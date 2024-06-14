import { Team as ApiTeam } from "../models/api-football/team";
import { Stage } from "../models/database/game";
import { Team as SqlTeam } from "../models/database/team";

export class Mapper {
  public static fromApiNameToSqlNames(apiTeam: ApiTeam): SqlTeam {
    switch (apiTeam) {
      case "France":
        return "France";
      case "Germany":
        return "Allemagne";
      case "Portugal":
        return "Portugal";
      case "Hungary":
        return "Hongrie";
      case "Spain":
        return "Espagne";
      case "Sweden":
        return "Suède";
      case "Poland":
        return "Pologne";
      case "Slovakia":
        return "Slovaquie";
      case "Scotland":
        return "Écosse";
      case "Czech Republic":
        return "République tchèque";
      case "Croatia":
        return "Croatie";
      case "England":
        return "Angleterre";
      case "Austria":
        return "Autriche";
      case "Netherlands":
        return "Pays-Bas";
      case "Ukraine":
        return "Ukraine";
      case "Serbia":
        return "Serbie";
      case "Denmark":
        return "Danemark";
      case "Finland":
        return "Finlande";
      case "Belgium":
        return "Belgique";
      case "Russia":
        return "Russie";
      case "Turkey":
        return "Turquie";
      case "Wales":
        return "Pays de Galles";
      case "Italy":
        return "Italie";
      case "Switzerland":
        return "Suisse";
      case "Albania":
        return "Albanie";
      case "Georgia":
        return "Géorgie";
      case "Romania":
        return "Roumanie";
      case "Slovenia":
        return "Slovénie";
      default:
        return apiTeam;
    }
  }

  public static fromMatchDateToStage(matchDate: Date): Stage {
    if (new Date(matchDate).getTime() < new Date("2024-06-27").getTime()) {
      return "group";
    } else if (
      new Date(matchDate).getTime() < new Date("2024-07-04").getTime()
    ) {
      return "sixteen";
    } else if (
      new Date(matchDate).getTime() < new Date("2024-07-08").getTime()
    ) {
      return "quarter";
    } else if (
      new Date(matchDate).getTime() < new Date("2024-07-12").getTime()
    ) {
      return "semi";
    } else {
      return "final";
    }
  }
}
