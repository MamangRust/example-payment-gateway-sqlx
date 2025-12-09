import {
  CreateSaldo,
  FindAllSaldo,
  FindByIdSaldo,
  UpdateSaldo,
} from "@/types/domain/request";
import {
  ApiResponseMonthSaldoBalances,
  ApiResponseMonthTotalSaldo,
  ApiResponsePaginationSaldo,
  ApiResponsePaginationSaldoDeleteAt,
  ApiResponseSaldo,
  ApiResponseYearSaldoBalances,
  ApiResponseYearTotalSaldo,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class SaldoCommand {
  async findMonthTotalBalance(
    accessToken: string,
    year: number,
    month: number,
  ): Promise<ApiResponseMonthTotalSaldo> {
    try {
      return await invoke("find_month_total_balance", {
        accessToken,
        year,
        month,
      });
    } catch (error) {
      console.error("Error in findMonthTotalBalance:", error);
      throw error;
    }
  }

  async findYearTotalBalance(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseYearTotalSaldo> {
    try {
      return await invoke("find_year_total_balance", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearTotalBalance:", error);
      throw error;
    }
  }

  async findMonthBalance(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseMonthSaldoBalances> {
    try {
      return await invoke("find_month_balance", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthBalance:", error);
      throw error;
    }
  }

  async findYearBalance(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseYearSaldoBalances> {
    try {
      return await invoke("find_year_balance", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearBalance:", error);
      throw error;
    }
  }

  async findAllSaldos(
    accessToken: string,
    req: FindAllSaldo,
  ): Promise<ApiResponsePaginationSaldo> {
    try {
      const response = await invoke<ApiResponsePaginationSaldo>(
        "find_all_saldos",
        {
          accessToken,
          req,
        },
      );

      console.log("response", response);

      return response;
    } catch (error) {
      console.error("Error in findAllSaldos:", error);
      throw error;
    }
  }

  async findByIdSaldo(
    accessToken: string,
    req: FindByIdSaldo,
  ): Promise<ApiResponseSaldo> {
    try {
      return await invoke("find_by_id_saldo", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByIdSaldo:", error);
      throw error;
    }
  }

  async findByActiveSaldo(
    accessToken: string,
    req: FindAllSaldo,
  ): Promise<ApiResponsePaginationSaldoDeleteAt> {
    try {
      return await invoke("find_by_active_saldo", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByActiveSaldo:", error);
      throw error;
    }
  }

  async createSaldo(
    accessToken: string,
    req: CreateSaldo,
  ): Promise<ApiResponseSaldo> {
    try {
      return await invoke("create_saldo", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in createSaldo:", error);
      throw error;
    }
  }

  async updateSaldo(
    accessToken: string,
    req: UpdateSaldo,
  ): Promise<ApiResponseSaldo> {
    try {
      return await invoke("update_saldo", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in updateSaldo:", error);
      throw error;
    }
  }

  async trashedSaldo(
    accessToken: string,
    req: FindByIdSaldo,
  ): Promise<ApiResponseSaldo> {
    try {
      return await invoke("trashed_saldo", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in trashedSaldo:", error);
      throw error;
    }
  }
}

export default new SaldoCommand();
