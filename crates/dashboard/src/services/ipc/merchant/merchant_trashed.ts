import {
  RestoreMerchantTrashed,
  DeletePermanentMerchant,
  FindAllMerchantTrashed,
} from "@/types/domain/request";
import {
  ApiResponseMerchant,
  ApiResponseMerchantAll,
  ApiResponseMerchantDelete,
  ApiResponsePaginationMerchantDeleteAt,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class MerchantTrashedCommand {
  async findAllMerchantsTrashed(
    accessToken: string,
    req: FindAllMerchantTrashed,
  ): Promise<ApiResponsePaginationMerchantDeleteAt> {
    try {
      return await invoke("find_all_merchants_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in findAllMerchantsTrashed:", error);
      throw error;
    }
  }

  async restoreMerchantTrashed(
    accessToken: string,
    req: RestoreMerchantTrashed,
  ): Promise<ApiResponseMerchant> {
    try {
      return await invoke("restore_merchant_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in restoreMerchantTrashed:", error);
      throw error;
    }
  }

  async deletePermanentMerchant(
    accessToken: string,
    req: DeletePermanentMerchant,
  ): Promise<ApiResponseMerchantDelete> {
    try {
      return await invoke("delete_permanent_merchant", { accessToken, req });
    } catch (error) {
      console.error("Error in deletePermanentMerchant:", error);
      throw error;
    }
  }

  async restoreMerchantAllTrashed(
    accessToken: string,
  ): Promise<ApiResponseMerchantAll> {
    try {
      return await invoke("restore_merchant_all_trashed", { accessToken });
    } catch (error) {
      console.error("Error in restoreMerchantAllTrashed:", error);
      throw error;
    }
  }

  async deletePermanentAllMerchant(
    accessToken: string,
  ): Promise<ApiResponseMerchantAll> {
    try {
      return await invoke("delete_permanent_all_merchant", { accessToken });
    } catch (error) {
      console.error("Error in deletePermanentAllMerchant:", error);
      throw error;
    }
  }
}
export default new MerchantTrashedCommand();
