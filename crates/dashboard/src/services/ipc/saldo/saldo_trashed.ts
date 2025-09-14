import {
  FindAllSaldo,
  RestoreSaldoTrashed,
  DeletePermanentSaldo,
} from "@/types/domain/request";
import {
  ApiResponseSaldo,
  ApiResponseSaldoAll,
  ApiResponseSaldoDelete,
  ApiResponsePaginationSaldoDeleteAt,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class SaldoTrashedCommand {
  async findAllSaldosTrashed(
    accessToken: string,
    req: FindAllSaldo,
  ): Promise<ApiResponsePaginationSaldoDeleteAt> {
    try {
      return await invoke("find_all_saldos_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in findAllSaldosTrashed:", error);
      throw error;
    }
  }

  async restoreSaldoTrashed(
    accessToken: string,
    req: RestoreSaldoTrashed,
  ): Promise<ApiResponseSaldo> {
    try {
      return await invoke("restore_saldo_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in restoreSaldoTrashed:", error);
      throw error;
    }
  }

  async deletePermanentSaldo(
    accessToken: string,
    req: DeletePermanentSaldo,
  ): Promise<ApiResponseSaldoDelete> {
    try {
      return await invoke("delete_permanent_saldo", { accessToken, req });
    } catch (error) {
      console.error("Error in deletePermanentSaldo:", error);
      throw error;
    }
  }

  async restoreSaldoAllTrashed(
    accessToken: string,
  ): Promise<ApiResponseSaldoAll> {
    try {
      return await invoke("restore_saldo_all_trashed", { accessToken });
    } catch (error) {
      console.error("Error in restoreSaldoAllTrashed:", error);
      throw error;
    }
  }

  async deletePermanentAllSaldo(
    accessToken: string,
  ): Promise<ApiResponseSaldoAll> {
    try {
      return await invoke("delete_permanent_all_saldo", { accessToken });
    } catch (error) {
      console.error("Error in deletePermanentAllSaldo:", error);
      throw error;
    }
  }
}

export default new SaldoTrashedCommand();
