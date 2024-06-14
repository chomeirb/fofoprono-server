import { Team } from "./team";

export interface Game {
  id: number;
  time: Date;
  stage: Stage;

  team_home: Team;
  team_away: Team;

  odds_home?: number;
  odds_draw?: number;
  odds_away?: number;

  competition_id: number;
  external_api_id: number;
}

export type Stage = "group" | "sixteen" | "quarter" | "semi" | "final";
