use diesel::prelude::*;
use uuid::Uuid;

use crate::models::message::DiscordMessage;
use crate::models::VestibuleUserRecord;

#[derive(Debug, Clone, Queryable, Selectable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name = crate::diesel_schema::skills)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub embedding: Option<pgvector::Vector>,
}

#[derive(
    Debug, Clone, Queryable, Selectable, Insertable, AsChangeset, Associations, Identifiable,
)]
#[diesel(table_name = crate::diesel_schema::user_skills)]
#[diesel(belongs_to(VestibuleUserRecord, foreign_key = user_id))]
#[diesel(belongs_to(Skill, foreign_key = skill_id))]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserSkill {
    pub id: Uuid,
    pub user_id: String,
    pub skill_id: Uuid,
    pub level: i16,
    pub llm_context: Option<String>,
}

#[derive(
    Debug, Clone, Queryable, Selectable, Insertable, AsChangeset, Associations, Identifiable,
)]
#[diesel(table_name = crate::diesel_schema::user_skill_evidence)]
#[diesel(belongs_to(UserSkill, foreign_key = user_skill_id))]
#[diesel(belongs_to(DiscordMessage, foreign_key = message_id))]
#[diesel(primary_key(user_skill_id, message_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserSkillEvidence {
    pub user_skill_id: Uuid,
    pub message_id: String,
    pub weight: f32,
    pub reasoning: String,
}
