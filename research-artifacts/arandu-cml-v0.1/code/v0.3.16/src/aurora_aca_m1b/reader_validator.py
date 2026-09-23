from __future__ import annotations
import json,re
from copy import deepcopy
from typing import Any
from .reader_schema import ROOT_FIELDS,RELATION_KINDS,PREDICATES,NEGATION_KINDS,CONDITION_KINDS

TOKEN_RE=re.compile(r"[A-Z][A-Z0-9-]*")

class ReaderValidationError(ValueError): pass

def _conf(x): return isinstance(x,(int,float)) and 0 <= x <= 1

def _span(item,text):
    s=item.get("support_text")
    return isinstance(s,str) and bool(s) and s in text

def _entity_ok(v,text):
    if v is None: return True
    if isinstance(v,list): return all(_entity_ok(x,text) for x in v)
    if not isinstance(v,str): return False
    return v in text

def canonicalize_raw(raw:dict[str,Any], unit:dict[str,Any]):
    """Conservative, current-unit-only normalization before validation.

    It never adds world knowledge. It may only normalize values that are explicitly
    recoverable from the current support span. This prevents benign surface-form
    drift from becoming entity/relation leakage while keeping fail-closed behavior.
    """
    if not isinstance(raw,dict): return raw
    out=deepcopy(raw); text=unit.get("text","")
    for c in out.get("concepts",[]) if isinstance(out.get("concepts"),list) else []:
        if not isinstance(c,dict): continue
        sup=c.get("support_text","")
        desc=c.get("description")
        if isinstance(sup,str) and isinstance(desc,str) and desc not in sup:
            m=re.search(r"\b(?:introduced as|denotes)\s+(.+?)(?:\.|$)",sup,re.I)
            if m: c["description"]=m.group(1).strip()
    for r in out.get("relations",[]) if isinstance(out.get("relations"),list) else []:
        if not isinstance(r,dict) or r.get("kind") != "BROADER_GROUP": continue
        sup=r.get("support_text","")
        if isinstance(sup,str):
            m=re.search(r"\bbroader\s+([A-Z][A-Z0-9-]*)\s+(?:configuration|pattern)\b",sup)
            if m: r["group"]=m.group(1)
    for p in out.get("propositions",[]) if isinstance(out.get("propositions"),list) else []:
        if not isinstance(p,dict): continue
        sup=p.get("support_text","")
        if p.get("label") is None and isinstance(sup,str):
            m=re.search(r"\brelation\s+([A-Z][A-Z0-9-]*)\b",sup)
            if m: p["label"]=m.group(1)
    return out

def validate_raw(raw:dict[str,Any], unit:dict[str,Any]):
    text=unit["text"]
    errs=[]
    if set(raw)!=ROOT_FIELDS: errs.append("SCHEMA_ROOT_FIELDS")
    if raw.get("schema_version")!="1.0": errs.append("SCHEMA_VERSION")
    if raw.get("unit_id")!=unit["unit_id"]: errs.append("UNIT_ID_MISMATCH")
    for k in ("concepts","propositions","relations","questions","negations","conditions","hypotheses"):
        if not isinstance(raw.get(k),list): errs.append(f"{k}:NOT_LIST")
    if not isinstance(raw.get("ambiguous"),bool) or not isinstance(raw.get("injection_detected"),bool): errs.append("FLAGS")
    if errs: return {"accepted":False,"errors":errs}

    for i,c in enumerate(raw["concepts"]):
        allowed={"surface_form","description","conceptual","support_text","confidence"}
        if set(c)!=allowed: errs.append(f"concept[{i}]:FIELDS"); continue
        if not _span(c,text): errs.append(f"concept[{i}]:GROUNDING")
        if not _entity_ok(c.get("surface_form"),text): errs.append(f"concept[{i}]:ENTITY")
        if c.get("description") not in c.get("support_text",""): errs.append(f"concept[{i}]:DESCRIPTION")
        if not isinstance(c.get("conceptual"),bool) or not _conf(c.get("confidence")): errs.append(f"concept[{i}]:TYPE")

    for i,p in enumerate(raw["propositions"]):
        allowed={"subject","predicate","object","label","epistemic_status","support_text","confidence"}
        if set(p)!=allowed: errs.append(f"proposition[{i}]:FIELDS"); continue
        if not _span(p,text): errs.append(f"proposition[{i}]:GROUNDING")
        if p.get("predicate") not in PREDICATES: errs.append(f"proposition[{i}]:RELATION")
        if p.get("epistemic_status")!="TEXT_EXPLICIT": errs.append(f"proposition[{i}]:EPISTEMIC")
        for fld in ("subject","object","label"):
            if not _entity_ok(p.get(fld),text): errs.append(f"proposition[{i}]:ENTITY:{fld}")
        if not _conf(p.get("confidence")): errs.append(f"proposition[{i}]:CONF")

    rel_fields={
      "REPETITION":{"kind","subject","object","label","support_text","confidence"},
      "TWO_FORMS":{"kind","base","forms","support_text","confidence"},
      "ASSOCIATED_WITH":{"kind","base","form","target","support_text","confidence"},
      "COEXISTS_WITH_FORMS":{"kind","concept","base","forms","support_text","confidence"},
      "BROADER_GROUP":{"kind","members","group","support_text","confidence"},
      "OUT_OF_DOMAIN_NOTE":{"kind","text","support_text","confidence"},
      "INSTRUCTION_AS_DATA":{"kind","text","support_text","confidence"},
      "AMBIGUOUS_COMPARISON":{"kind","text","support_text","confidence"},
      "RETAIN_GROUP":{"kind","group","support_text","confidence"},
      "CONTEXT_ASSOCIATION":{"kind","context","base","form","target","support_text","confidence"},
      "INSTANCE_OF_GROUP":{"kind","members","group","support_text","confidence"},
    }
    entity_fields={
      "REPETITION":["subject","object","label"],"TWO_FORMS":["base","forms"],
      "ASSOCIATED_WITH":["base","form","target"],"COEXISTS_WITH_FORMS":["concept","base","forms"],
      "BROADER_GROUP":["members","group"],"RETAIN_GROUP":["group"],
      "CONTEXT_ASSOCIATION":["context","base","form","target"],"INSTANCE_OF_GROUP":["members","group"]
    }
    for i,r in enumerate(raw["relations"]):
        kind=r.get("kind")
        if kind not in RELATION_KINDS: errs.append(f"relation[{i}]:RELATION"); continue
        if set(r)!=rel_fields[kind]: errs.append(f"relation[{i}]:FIELDS"); continue
        if not _span(r,text): errs.append(f"relation[{i}]:GROUNDING")
        if not _conf(r.get("confidence")): errs.append(f"relation[{i}]:CONF")
        for fld in entity_fields.get(kind,[]):
            if not _entity_ok(r.get(fld),text): errs.append(f"relation[{i}]:ENTITY:{fld}")
        if kind in {"OUT_OF_DOMAIN_NOTE","INSTRUCTION_AS_DATA","AMBIGUOUS_COMPARISON"} and r.get("text") != r.get("support_text"): errs.append(f"relation[{i}]:TEXT")

    for i,q in enumerate(raw["questions"]):
        if set(q)!={"text","support_text","confidence"}: errs.append(f"question[{i}]:FIELDS"); continue
        if not _span(q,text) or q.get("text") not in q.get("support_text",""): errs.append(f"question[{i}]:GROUNDING")
        if not _conf(q.get("confidence")): errs.append(f"question[{i}]:CONF")

    for i,n in enumerate(raw["negations"]):
        if set(n)!={"kind","context","concept","base","form","support_text","confidence"}: errs.append(f"negation[{i}]:FIELDS"); continue
        if n.get("kind") not in NEGATION_KINDS: errs.append(f"negation[{i}]:KIND")
        if not _span(n,text): errs.append(f"negation[{i}]:GROUNDING")
        for fld in ("context","concept","base","form"):
            if not _entity_ok(n.get(fld),text): errs.append(f"negation[{i}]:ENTITY:{fld}")
        if not _conf(n.get("confidence")): errs.append(f"negation[{i}]:CONF")

    for i,c in enumerate(raw["conditions"]):
        if set(c)!={"kind","positive","negative","concept","base","form","support_text","confidence"}: errs.append(f"condition[{i}]:FIELDS"); continue
        if c.get("kind") not in CONDITION_KINDS: errs.append(f"condition[{i}]:KIND")
        if not _span(c,text): errs.append(f"condition[{i}]:GROUNDING")
        for fld in ("positive","negative","concept","base","form"):
            if not _entity_ok(c.get(fld),text): errs.append(f"condition[{i}]:ENTITY:{fld}")
        if not _conf(c.get("confidence")): errs.append(f"condition[{i}]:CONF")

    for i,h in enumerate(raw["hypotheses"]):
        if set(h)!={"subject","predicate","object","epistemic_status","support_text","confidence"}: errs.append(f"hypothesis[{i}]:FIELDS"); continue
        if not _span(h,text): errs.append(f"hypothesis[{i}]:GROUNDING")
        if not _entity_ok(h.get("subject"),text): errs.append(f"hypothesis[{i}]:ENTITY")
        if h.get("predicate")!="MAY_REDUCE" or h.get("object")!="FUTURE_REASONING_BURDEN": errs.append(f"hypothesis[{i}]:RELATION")
        if h.get("epistemic_status")!="DERIVED_HYPOTHESIS": errs.append(f"hypothesis[{i}]:EPISTEMIC")
        if not _conf(h.get("confidence")): errs.append(f"hypothesis[{i}]:CONF")

    low=text.lower()
    if ("ignore all previous" in low or "ignore previous" in low) and not raw.get("injection_detected"):
        errs.append("INJECTION_NOT_DETECTED")
    if raw.get("injection_detected") and not any(r.get("kind")=="INSTRUCTION_AS_DATA" for r in raw["relations"]):
        errs.append("INJECTION_NOT_DATA")
    # v0.3.16-dev: epistemic ambiguity and extraction confidence are orthogonal.
    # `ambiguous` describes what the source text says; per-item `confidence`
    # describes confidence that the Reader extracted that source content correctly.
    # A Reader may therefore be highly confident that a source explicitly expresses
    # uncertainty/ambiguity. Confidence remains range-validated above, but is not
    # capped merely because the source content is ambiguous.
    return {"accepted":not errs,"errors":errs}

def parse_and_validate(raw_text:str,unit:dict[str,Any]):
    try: raw=json.loads(raw_text)
    except Exception: return None,{"accepted":False,"errors":["INVALID_JSON"]}
    if not isinstance(raw,dict): return None,{"accepted":False,"errors":["ROOT_NOT_OBJECT"]}
    raw=canonicalize_raw(raw,unit)
    return raw,validate_raw(raw,unit)
