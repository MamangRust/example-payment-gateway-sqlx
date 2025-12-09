import {
  FindAllTransaction,
  RestoreTransactionTrashed,
  DeletePermanentTransaction,
} from "@/types/domain/request";
import {
  ApiResponseTransaction,
  ApiResponseTransactionAll,
  ApiResponseTransactionDelete,
  ApiResponsePaginationTransactionDeleteAt,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class TransactionTrashedCommand {
  async findAllTransactionsTrashed(
    accessToken: string,
    req: FindAllTransaction,
  ): Promise<ApiResponsePaginationTransactionDeleteAt> {
    try {
      return await invoke("find_all_transactions_trashed", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findAllTransactionsTrashed:", error);
      throw error;
    }
  }

  async restoreTransactionTrashed(
    accessToken: string,
    req: RestoreTransactionTrashed,
  ): Promise<ApiResponseTransaction> {
    try {
      return await invoke("restore_transaction_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in restoreTransactionTrashed:", error);
      throw error;
    }
  }

  async deletePermanentTransaction(
    accessToken: string,
    req: DeletePermanentTransaction,
  ): Promise<ApiResponseTransactionDelete> {
    try {
      return await invoke("delete_permanent_transaction", { accessToken, req });
    } catch (error) {
      console.error("Error in deletePermanentTransaction:", error);
      throw error;
    }
  }

  async restoreTransactionAllTrashed(
    accessToken: string,
  ): Promise<ApiResponseTransactionAll> {
    try {
      return await invoke("restore_transaction_all_trashed", { accessToken });
    } catch (error) {
      console.error("Error in restoreTransactionAllTrashed:", error);
      throw error;
    }
  }

  async deletePermanentAllTransaction(
    accessToken: string,
  ): Promise<ApiResponseTransactionAll> {
    try {
      return await invoke("delete_permanent_all_transaction", { accessToken });
    } catch (error) {
      console.error("Error in deletePermanentAllTransaction:", error);
      throw error;
    }
  }
}

export default new TransactionTrashedCommand();
