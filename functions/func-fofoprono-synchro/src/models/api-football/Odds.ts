export interface Odds {
  fixture: {
    id: number;
  };
  bookmakers: {
    bets: {
      values: {
        value: MatchWinnerOptions;
        odd: string;
      }[];
    }[];
  }[];
}

export type MatchWinnerOptions = "Home" | "Draw" | "Away";
