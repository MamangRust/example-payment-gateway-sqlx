import {
  FindAllWithdraw,
  RestoreWithdrawTrashed,
  DeletePermanentWithdraw,
} from "@/types/domain/request";
import {
  ApiResponseWithdraw,
  ApiResponseWithdrawAll,
  ApiResponseWithdrawDelete,
  ApiResponsePaginationWithdrawDeleteAt,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class WithdrawTrashedCommand {
  async findAllWithdrawsTrashed(
    accessToken: string,
    req: FindAllWithdraw,
  ): Promise<ApiResponsePaginationWithdrawDeleteAt> {
    try {
      return await invoke("find_all_withdraws_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in findAllWithdrawsTrashed:", error);
      throw error;
    }
  }

  async restoreWithdrawTrashed(
    accessToken: string,
    req: RestoreWithdrawTrashed,
  ): Promise<ApiResponseWithdraw> {
    try {
      return await invoke("restore_withdraw_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in restoreWithdrawTrashed:", error);
      throw error;
    }
  }

  async deletePermanentWithdraw(
    accessToken: string,
    req: DeletePermanentWithdraw,
  ): Promise<ApiResponseWithdrawDelete> {
    try {
      return await invoke("delete_permanent_withdraw", { accessToken, req });
    } catch (error) {
      console.error("Error in deletePermanentWithdraw:", error);
      throw error;
    }
  }

  async restoreWithdrawAllTrashed(
    accessToken: string,
  ): Promise<ApiResponseWithdrawAll> {
    try {
      return await invoke("restore_withdraw_all_trashed", { accessToken });
    } catch (error) {
      console.error("Error in restoreWithdrawAllTrashed:", error);
      throw error;
    }
  }

  async deletePermanentAllWithdraw(
    accessToken: string,
  ): Promise<ApiResponseWithdrawAll> {
    try {
      return await invoke("delete_permanent_all_withdraw", { accessToken });
    } catch (error) {
      console.error("Error in deletePermanentAllWithdraw:", error);
      throw error;
    }
  }
}
export default new WithdrawTrashedCommand();
