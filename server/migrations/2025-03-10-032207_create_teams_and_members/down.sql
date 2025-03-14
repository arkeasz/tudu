ALTER TABLE projects DROP FOREIGN KEY fk_project_team;
ALTER TABLE projects DROP COLUMN team_id;

DROP TABLE IF EXISTS team_members;
DROP TABLE IF EXISTS teams;
