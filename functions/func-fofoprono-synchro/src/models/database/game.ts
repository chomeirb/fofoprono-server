export interface Game {
  id: number;
  time: Date;
  stage: Stage;

  team_home: string;
  team_away: string;

  odds_home?: number;
  odds_draw?: number;
  odds_away?: number;

  competition_id: number;
  external_api_id: number;
}

export type Stage = "Group" | "Sixteen" | "Quarter" | "Semi" | "Final";
