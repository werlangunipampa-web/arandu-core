from __future__ import annotations
import hashlib,json
from typing import Any

def _span(unit,item):
    text=unit["text"]; s=item["support_text"]; i=text.index(s)
    return [{"unit_id":unit["unit_id"],"start":i,"end":i+len(s),"text":s}]

def build_candidate(raw:dict[str,Any],unit:dict[str,Any],provider_meta:dict[str,Any],raw_text:str):
    uid=unit["unit_id"]
    out={"candidate_id":f"CE-{uid}","source_unit":uid,"concepts":[],"propositions":[],"relations":[],"questions":[],"negations":[],"conditions":[],"hypotheses":[],"ambiguous":raw["ambiguous"],"injection":raw["injection_detected"],"grounding_complete":True,"reader_provider":provider_meta.get("provider"),"reader_model":provider_meta.get("model"),"reader_output_hash":hashlib.sha256(raw_text.encode()).hexdigest()}
    for c in raw["concepts"]:
        out["concepts"].append({"label":c["surface_form"],"description":c["description"],"conceptual":c["conceptual"],"support_spans":_span(unit,c),"confidence":c["confidence"]})
    for p in raw["propositions"]:
        out["propositions"].append({"subject":p["subject"],"predicate":p["predicate"],"object":p["object"],"label":p["label"],"support_spans":_span(unit,p),"epistemic_status":p["epistemic_status"]})
    for r in raw["relations"]:
        kind=r["kind"]
        x={"kind":kind,"support_spans":_span(unit,r)}
        if kind=="REPETITION": x.update(subject=r["subject"],object=r["object"],label=r["label"])
        elif kind=="TWO_FORMS": x.update(base=r["base"],forms=[f.upper() for f in r["forms"]])
        elif kind=="ASSOCIATED_WITH": x.update(base=r["base"],form=r["form"],target=r["target"])
        elif kind=="COEXISTS_WITH_FORMS": x.update(concept=r["concept"],base=r["base"],forms=[f.upper() for f in r["forms"]])
        elif kind=="BROADER_GROUP": x.update(members=r["members"],group=r["group"])
        elif kind in {"OUT_OF_DOMAIN_NOTE","INSTRUCTION_AS_DATA","AMBIGUOUS_COMPARISON"}: x.update(text=r["text"])
        elif kind=="RETAIN_GROUP": x.update(group=r["group"])
        elif kind=="CONTEXT_ASSOCIATION": x.update(context=r["context"],base=r["base"],form=r["form"],target=r["target"])
        elif kind=="INSTANCE_OF_GROUP": x.update(members=r["members"],group=r["group"])
        out["relations"].append(x)
    for q in raw["questions"]: out["questions"].append({"text":q["text"],"support_spans":_span(unit,q)})
    for n in raw["negations"]: out["negations"].append({"kind":n["kind"],"context":n["context"],"concept":n["concept"],"base":n["base"],"form":n["form"],"support_spans":_span(unit,n)})
    for c in raw["conditions"]: out["conditions"].append({"kind":c["kind"],"positive":c["positive"],"negative":c["negative"],"concept":c["concept"],"base":c["base"],"form":c["form"],"support_spans":_span(unit,c)})
    for h in raw["hypotheses"]: out["hypotheses"].append({"subject":h["subject"],"predicate":h["predicate"],"object":h["object"],"support_spans":_span(unit,h),"epistemic_status":h["epistemic_status"]})
    return out
