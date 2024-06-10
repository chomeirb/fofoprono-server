import { Game } from "../models/api-football/Game";
import * as dotenv from "dotenv";

dotenv.config();

const leagueId = 4;
const season = 2024;
const apiBaseUrl = "https://api-football-v1.p.rapidapi.com/v3";
const apiKey = process.env.API_FOOTBALL_KEY;

export class ApiFootballHelper {
  public static async getGames() {
    const params = {
      league: leagueId.toString(),
      season: season.toString(),
    };

    return await this.fetch<Game>("fixtures", params);
  }

  public static async getOdds() {
    const params = {
      league: leagueId.toString(),
      season: season.toString(),
    };

    return await this.fetch<Game>("odds", params);
  }

  private static async fetch<T>(
    endpoint: string,
    params: { [key: string]: string }
  ) {
    const result = await fetch(buildUrl(`${apiBaseUrl}/${endpoint}`, params), {
      method: "GET",
      headers: {
        "x-rapidapi-key": apiKey,
      },
    });

    const deserializedGames: T[] = (await result.json()).response.map(
      (game: any) => {
        return game as T;
      }
    );
    return deserializedGames;
  }
}

function buildUrl(url: string, params: { [key: string]: string }) {
  const searchParams = new URLSearchParams(params);
  return `${url}?${searchParams.toString()}`;
}
