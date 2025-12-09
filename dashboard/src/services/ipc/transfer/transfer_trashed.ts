import {
  FindAllTransfer,
  RestoreTransferTrashed,
  DeletePermanentTransfer,
} from "@/types/domain/request";
import {
  ApiResponseTransfer,
  ApiResponseTransferAll,
  ApiResponseTransferDelete,
  ApiResponsePaginationTransferDeleteAt,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class TransferTrashedCommand {
  async findAllTransfersTrashed(
    accessToken: string,
    req: FindAllTransfer,
  ): Promise<ApiResponsePaginationTransferDeleteAt> {
    try {
      return await invoke("find_all_transfers_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in findAllTransfersTrashed:", error);
      throw error;
    }
  }

  async restoreTransferTrashed(
    accessToken: string,
    req: RestoreTransferTrashed,
  ): Promise<ApiResponseTransfer> {
    try {
      return await invoke("restore_transfer_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in restoreTransferTrashed:", error);
      throw error;
    }
  }

  async deletePermanentTransfer(
    accessToken: string,
    req: DeletePermanentTransfer,
  ): Promise<ApiResponseTransferDelete> {
    try {
      return await invoke("delete_permanent_transfer", { accessToken, req });
    } catch (error) {
      console.error("Error in deletePermanentTransfer:", error);
      throw error;
    }
  }

  async restoreTransferAllTrashed(
    accessToken: string,
  ): Promise<ApiResponseTransferAll> {
    try {
      return await invoke("restore_transfer_all_trashed", { accessToken });
    } catch (error) {
      console.error("Error in restoreTransferAllTrashed:", error);
      throw error;
    }
  }

  async deletePermanentAllTransfer(
    accessToken: string,
  ): Promise<ApiResponseTransferAll> {
    try {
      return await invoke("delete_permanent_all_transfer", { accessToken });
    } catch (error) {
      console.error("Error in deletePermanentAllTransfer:", error);
      throw error;
    }
  }
}

export default new TransferTrashedCommand();
