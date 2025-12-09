import {
  DeletePermanentTopup,
  FindAllTopup,
  RestoreTopupTrashed,
} from "@/types/domain/request/topup";
import {
  ApiResponseTopup,
  ApiResponseTopupAll,
  ApiResponseTopupDelete,
  ApiResponsePaginationTopupDeleteAt,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class TopupTrashedCommand {
  async findAllTopupsTrashed(
    accessToken: string,
    req: FindAllTopup,
  ): Promise<ApiResponsePaginationTopupDeleteAt> {
    try {
      return await invoke("find_all_topups_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in findAllTopupsTrashed:", error);
      throw error;
    }
  }

  async restoreTopupTrashed(
    accessToken: string,
    req: RestoreTopupTrashed,
  ): Promise<ApiResponseTopup> {
    try {
      return await invoke("restore_topup_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in restoreTopupTrashed:", error);
      throw error;
    }
  }

  async deletePermanentTopup(
    accessToken: string,
    req: DeletePermanentTopup,
  ): Promise<ApiResponseTopupDelete> {
    try {
      return await invoke("delete_permanent_topup", { accessToken, req });
    } catch (error) {
      console.error("Error in deletePermanentTopup:", error);
      throw error;
    }
  }

  async restoreTopupAllTrashed(
    accessToken: string,
  ): Promise<ApiResponseTopupAll> {
    try {
      return await invoke("restore_topup_all_trashed", { accessToken });
    } catch (error) {
      console.error("Error in restoreTopupAllTrashed:", error);
      throw error;
    }
  }

  async deletePermanentAllTopup(
    accessToken: string,
  ): Promise<ApiResponseTopupAll> {
    try {
      return await invoke("delete_permanent_all_topup", { accessToken });
    } catch (error) {
      console.error("Error in deletePermanentAllTopup:", error);
      throw error;
    }
  }
}
export default new TopupTrashedCommand();
