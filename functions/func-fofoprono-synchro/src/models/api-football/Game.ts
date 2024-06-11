export interface Game {
  fixture: {
    id: number;
    date: Date;
  };
  teams: {
    home: {
      name: string;
    };
    away: {
      name: string;
    };
  };
}
