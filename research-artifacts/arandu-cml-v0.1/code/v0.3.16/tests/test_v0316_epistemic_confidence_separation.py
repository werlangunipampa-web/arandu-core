import json
from aurora_aca_m1b.reader_validator import parse_and_validate

def _base(text, confidence, ambiguous=True):
    return {
      'schema_version':'1.0','unit_id':'VTEST','concepts':[],'propositions':[],
      'relations':[{'kind':'AMBIGUOUS_COMPARISON','text':text,'support_text':text,'confidence':confidence}],
      'questions':[],'negations':[],'conditions':[],'hypotheses':[],
      'ambiguous':ambiguous,'injection_detected':False
    }

def test_high_confidence_ambiguous_content_is_valid():
    text='The comparison of RAVA with TENO rather than MIKA remains uncertain.'
    raw=_base(text,0.99)
    _,vr=parse_and_validate(json.dumps(raw),{'unit_id':'VTEST','text':text})
    assert vr['accepted'], vr['errors']
    assert 'AMBIGUITY_CONFIDENCE_CAP' not in vr['errors']

def test_confidence_is_still_range_checked():
    text='The comparison of RAVA with TENO rather than MIKA remains uncertain.'
    raw=_base(text,1.2)
    _,vr=parse_and_validate(json.dumps(raw),{'unit_id':'VTEST','text':text})
    assert not vr['accepted']
    assert 'relation[0]:CONF' in vr['errors']

def test_ordinary_high_confidence_remains_valid():
    text='The quiet RAVA form is associated with TENO.'
    raw={
      'schema_version':'1.0','unit_id':'VTEST','concepts':[],'propositions':[],
      'relations':[{'kind':'ASSOCIATED_WITH','base':'RAVA','form':'quiet','target':'TENO','support_text':text,'confidence':0.99}],
      'questions':[],'negations':[],'conditions':[],'hypotheses':[],
      'ambiguous':False,'injection_detected':False
    }
    _,vr=parse_and_validate(json.dumps(raw),{'unit_id':'VTEST','text':text})
    assert vr['accepted'], vr['errors']

def test_change_is_validator_only_not_candidate_learning_logic():
    from pathlib import Path
    here=Path(__file__).resolve().parents[1]
    import hashlib
    expected={
      'src/aurora_aca_m1b/engine.py':'d552ec165c120975629cd807362e9313c4b82db15536b8c8b1170264c0f75aa3',
      'src/aurora_aca_m1b/proposer.py':'04c83753b5262dbfe6568e31ed2c21def95a29194675546b90687d7ed93cb01d',
      'src/aurora_aca_m1b/literal_rescue.py':'f278785f5aab2fdeaba783412b0fd66c34b78cf2e2ea638185d9534b547cb884',
      'src/aurora_aca_m1b/candidate_builder.py':'d385db9593fb94efe6309ff6663818c44fd9792be0150a51487c5869a36eb87b',
    }
    for rel,exp in expected.items():
        assert hashlib.sha256((here/rel).read_bytes()).hexdigest()==exp
