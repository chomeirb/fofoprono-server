import { Game } from "../models/api-football/game.js";
import { Odds } from "../models/api-football/odds.js";
import dotenv from "dotenv";

dotenv.config();
const leagueId = 4;
const season = 2024;
const apiBaseUrl = "https://api-football-v1.p.rapidapi.com/v3";
const apiKey = process.env.API_FOOTBALL_KEY;

export class ApiFootballClient {
  public static async getGames() {
    const params = {
      league: leagueId.toString(),
      season: season.toString(),
      from: formatDate(new Date()),
      to: formatDate(new Date(new Date().setDate(new Date().getDate() + 40))),
    };

    console.log(params);
    return await this.fetch<Game>("fixtures", params);
  }

  public static async getOdds(bookmakerId: number) {
    const params = {
      league: leagueId.toString(),
      season: season.toString(),
      bookmaker: bookmakerId.toString(),
    };

    return await this.fetch<Odds>("odds", params, true);
  }

  private static async fetch<T>(
    endpoint: string,
    params: { [key: string]: string },
    pageable: boolean = false
  ) {
    let current: number = 1;
    let total: number = 1;
    const data: T[] = [];
    console.log(params);

    while (current <= total) {
      if (pageable) {
        params.page = current.toString();
      }
      console.log(params);

      const result = await fetch(
        buildUrl(`${apiBaseUrl}/${endpoint}`, params),
        {
          method: "GET",
          headers: {
            "x-rapidapi-key": apiKey,
          },
        }
      );

      const deserializedData: {
        paging: {
          current: number;
          total: number;
        };
        response: T[];
      } = await result.json();

      data.push(...deserializedData.response);

      current = deserializedData.paging.current + 1;
      total = deserializedData.paging.total;
    }
    return data;
  }
}
function buildUrl(url: string, params: { [key: string]: string }) {
  const searchParams = new URLSearchParams(params);
  const u = `${url}?${searchParams.toString()}`;
  console.log(u);
  return u;
}

function formatDate(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0"); // Months are zero-based
  const day = String(date.getDate()).padStart(2, "0");

  return `${year}-${month}-${day}`;
}
