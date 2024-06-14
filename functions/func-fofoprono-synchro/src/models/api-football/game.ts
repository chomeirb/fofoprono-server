import { Team } from "./team";

export interface Game {
  fixture: {
    id: number;
    date: Date;
  };
  teams: {
    home: {
      name: Team;
    };
    away: {
      name: Team;
    };
  };
}
