pub struct TraitDesc {
    pub name: &'static str,
    pub description: &'static str,
    pub high: &'static str,
    pub low: &'static str,
}

pub const HEXACO: &[TraitDesc] = &[
    TraitDesc {
        name: "honesty",
        description: "Sincerity, fairness, lack of greed/entitlement",
        high: "Genuine, doesn't manipulate or exploit, modest, not status-driven",
        low: "Flatters for gain, bends rules for advantage, feels entitled, status-seeking",
    },
    TraitDesc {
        name: "emotionality",
        description: "Emotional reactivity, volatility, anxiety",
        high: "Gets heated easily, expresses strong emotions, anxious, reactive to stress",
        low: "Calm under pressure, emotionally stable, rarely gets worked up",
    },
    TraitDesc {
        name: "extraversion",
        description: "Social confidence, enthusiasm, enjoyment of interaction",
        high: "Initiates conversations, energetic, confident, enjoys group discussions",
        low: "Quiet, reserved, avoids spotlight, prefers observing over participating",
    },
    TraitDesc {
        name: "agreeableness",
        description: "Forgiveness, gentleness, patience, willingness to compromise",
        high: "Lets slights go, avoids conflict, patient with others, flexible",
        low: "Holds grudges, critical, quick to argue, stubborn in disagreements",
    },
    TraitDesc {
        name: "conscientiousness",
        description: "Organization, diligence, perfectionism, prudence",
        high: "Plans carefully, thorough, disciplined, considers consequences",
        low: "Disorganized, impulsive, cuts corners, acts without planning",
    },
    TraitDesc {
        name: "openness_to_experience",
        description: "Curiosity, creativity, aesthetic appreciation, unconventionality",
        high: "Explores ideas, appreciates art/beauty, creative, embraces unusual concepts",
        low: "Practical-focused, conventional, uninterested in abstract/artistic topics",
    },
];

pub const BEHAVIORAL: &[TraitDesc] = &[
    TraitDesc {
        name: "agency",
        description: "Follow-through on stated commitments and self-initiated action",
        high:
            "Does what they say, self-directed, drives progress and works on projects proactively",
        low: "Makes promises but doesn't deliver, passive, waits for others",
    },
    TraitDesc {
        name: "achievement",
        description: "Track record of completed, impactful projects or accomplishments",
        high: "Has shipped projects, can point to concrete outcomes",
        low: "Many started projects, few finished; talks about ideas without execution",
    },
    TraitDesc {
        name: "influence",
        description: "Social influence and authority in conversations (behavioral, not role-based)",
        high: "Others defer to their opinions, shapes discussion direction, respected",
        low: "Opinions ignored, follows rather than leads discussions",
    },
    TraitDesc {
        name: "sarcasm",
        description: "Frequency and intensity of ironic/sarcastic communication",
        high: "Often says the opposite of what they mean, dry humor, mocking tone",
        low: "Direct, literal communication, rare irony",
    },
    TraitDesc {
        name: "security",
        description: "Certainty vs. self-doubt in communication",
        high: "Confident assertions, rarely hedges, owns their opinions",
        low: "Frequently hedges (\"I think maybe...\"), seeks validation, self-deprecating",
    },
    TraitDesc {
        name: "self_reflection",
        description: "Explicit reconsideration of own beliefs, decisions, or growth.",
        high: "\"I used to think X but now...\", acknowledges mistakes, updates views",
        low: "Never revisits past positions, doesn't discuss personal growth",
    },
    TraitDesc {
        name: "technical_competence",
        description: "Quality of technical reasoning/solutions demonstrated",
        high: "Correct, nuanced technical explanations; solves problems efficiently",
        low: "Frequent errors, surface-level understanding, needs correction often",
    },
    TraitDesc {
        name: "busyness",
        description: "How occupied the person appears with projects, work, or life obligations",
        high: "Frequently mentions being busy, many concurrent commitments, limited availability",
        low: "Appears to have free time, few mentioned obligations, readily available",
    },
];

pub fn scoring_prompt() -> String {
    let mut out = String::from(
        "# Personality & Behavioral Scoring\n\n\
         Score the message on each trait using a 0.0 to 1.0 scale.\n\
         - Return null if there is no/insufficient evidence
         - Use values closer to extremes only with clear evidence\n\n\
         ## HEXACO Personality Traits\n\n",
    );

    for t in HEXACO {
        out.push_str(&format!(
            "- **{}**: {}\n  - High (→1.0): {}\n  - Low (→0.0): {}\n",
            t.name, t.description, t.high, t.low
        ));
    }

    out.push_str("## Behavioral Traits\n\n");

    for t in BEHAVIORAL {
        out.push_str(&format!(
            "- **{}**: {}\n  - High (→1.0): {}\n  - Low (→0.0): {}\n",
            t.name, t.description, t.high, t.low
        ));
    }

    out.push_str("## Expected Output Format\n\n```json\n{\n");

    for t in HEXACO.iter().chain(BEHAVIORAL.iter()) {
        out.push_str(&format!("  \"{}\": <0.0-1.0>,\n", t.name));
    }
    out.push_str("  \"activities\": [\"<extracted interests/hobbies>\"]\n}\n```");

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{BehavioralTraits, HexacoTraits};
    use std::collections::HashSet;

    fn struct_field_names<T: Default + serde::Serialize>() -> HashSet<String> {
        let value = serde_json::to_value(T::default()).unwrap();

        value.as_object().unwrap().keys().cloned().collect()
    }

    fn trait_names(traits: &[TraitDesc]) -> HashSet<String> {
        traits.iter().map(|t| t.name.to_string()).collect()
    }

    #[test]
    fn hexaco_fields_match_schema() {
        let struct_fields = struct_field_names::<HexacoTraits>();
        let schema_names = trait_names(HEXACO);

        assert_eq!(
            struct_fields,
            schema_names,
            "HexacoTraits fields don't match HEXACO schema.\n\
             In struct but not schema: {:?}\n\
             In schema but not struct: {:?}",
            struct_fields.difference(&schema_names).collect::<Vec<_>>(),
            schema_names.difference(&struct_fields).collect::<Vec<_>>()
        );
    }

    #[test]
    fn behavioral_fields_match_schema() {
        let struct_fields = struct_field_names::<BehavioralTraits>();
        let schema_names = trait_names(BEHAVIORAL);

        assert_eq!(
            struct_fields,
            schema_names,
            "BehavioralTraits fields don't match BEHAVIORAL schema.\n\
             In struct but not schema: {:?}\n\
             In schema but not struct: {:?}",
            struct_fields.difference(&schema_names).collect::<Vec<_>>(),
            schema_names.difference(&struct_fields).collect::<Vec<_>>()
        );
    }
}
