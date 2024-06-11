import { Game } from "../models/api-football/game.js";
import { Odds } from "../models/api-football/odds.js";
import dotenv from "dotenv";

dotenv.config();
const leagueId = 4;
const season = 2024;
const apiBaseUrl = "https://api-football-v1.p.rapidapi.com/v3";
const apiKey = process.env.API_FOOTBALL_KEY;

export class ApiFootballClient {
  public static async getGames(fromDate: Date, toDate: Date) {
    const params = {
      league: leagueId.toString(),
      season: season.toString(),
      from: formatDate(fromDate),
      to: formatDate(toDate),
    };

    return await this.fetch<Game>("fixtures", params);
  }

  public static async getOdds(bookmakerId: number) {
    const params = {
      league: leagueId.toString(),
      season: season.toString(),
      bookmaker: bookmakerId.toString(),
    };

    return await this.fetch<Odds>("odds", params);
  }

  private static async fetch<T>(
    endpoint: string,
    params: { [key: string]: string }
  ) {
    console.log(apiKey);
    const result = await fetch(buildUrl(`${apiBaseUrl}/${endpoint}`, params), {
      method: "GET",
      headers: {
        "x-rapidapi-key": apiKey,
      },
    });

    const deserializedData: T[] = (await result.json()).response.map(
      (game: any) => {
        return game as T;
      }
    );
    return deserializedData;
  }
}

function buildUrl(url: string, params: { [key: string]: string }) {
  const searchParams = new URLSearchParams(params);
  return `${url}?${searchParams.toString()}`;
}

function formatDate(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0"); // Months are zero-based
  const day = String(date.getDate()).padStart(2, "0");

  return `${year}-${month}-${day}`;
}
