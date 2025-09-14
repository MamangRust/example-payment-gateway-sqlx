import { FindAllCard, FindByIdCard } from "@/types/domain/request";
import {
  ApiResponseCard,
  ApiResponseCardAll,
  ApiResponsePaginationCardDeleteAt,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class CardTrashedCommand {
  async findAllCardsTrashed(
    accessToken: string,
    req: FindAllCard,
  ): Promise<ApiResponsePaginationCardDeleteAt> {
    try {
      return await invoke("find_all_cards_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in findAllCardsTrashed:", error);
      throw error;
    }
  }

  async restoreCardTrashed(
    accessToken: string,
    req: FindByIdCard,
  ): Promise<ApiResponseCard> {
    try {
      return await invoke("restore_card_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in restoreCardTrashed:", error);
      throw error;
    }
  }

  async deletePermanentCard(
    accessToken: string,
    req: FindByIdCard,
  ): Promise<ApiResponseCard> {
    try {
      return await invoke("delete_permanent_card", { accessToken, req });
    } catch (error) {
      console.error("Error in deletePermanentCard:", error);
      throw error;
    }
  }

  async restoreAllCardsTrashed(
    accessToken: string,
  ): Promise<ApiResponseCardAll> {
    try {
      return await invoke("restore_all_cards_trashed", { accessToken });
    } catch (error) {
      console.error("Error in restoreAllCardsTrashed:", error);
      throw error;
    }
  }

  async deletePermanentAllCards(
    accessToken: string,
  ): Promise<ApiResponseCardAll> {
    try {
      return await invoke("delete_permanent_all_cards", { accessToken });
    } catch (error) {
      console.error("Error in deletePermanentAllCards:", error);
      throw error;
    }
  }
}

export default new CardTrashedCommand();
