import myApi from "@/helpers/api";
import {
  CreateSaldo,
  FindAllSaldo,
  FindByCardNumberSaldo,
  FindByIdSaldo,
  TrashedSaldo,
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

class SaldoService {
  async findMonthTotalBalance(
    access_token: string,
    year: number,
    month: number,
  ): Promise<ApiResponseMonthTotalSaldo["data"]> {
    try {
      const response = await myApi.get("/saldos/stats/total-balance/monthly", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
          month,
        },
      });

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findYearTotalBalance(
    access_token: string,
    year: number,
  ): Promise<ApiResponseYearTotalSaldo["data"]> {
    try {
      const response = await myApi.get("/saldos/stats/total-balance/yearly", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
        },
      });

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findMonthBalance(
    access_token: string,
    year: number,
  ): Promise<ApiResponseMonthSaldoBalances["data"]> {
    try {
      const response = await myApi.get("/saldos/stats/balance/monthly", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
        },
      });

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findYearBalance(
    access_token: string,
    year: number,
  ): Promise<ApiResponseYearSaldoBalances["data"]> {
    try {
      const response = await myApi.get("/saldos/stats/balance/yearly", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
        },
      });

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findAllSaldos(
    access_token: string,
    req: FindAllSaldo,
  ): Promise<ApiResponsePaginationSaldo> {
    try {
      const response = await myApi.get("/saldos", {
        params: {
          page: req.page,
          page_size: req.page_size,
          search: req.search,
        },
        headers: { Authorization: `Bearer ${access_token}` },
      });

      if (response.status == 200) {
        return response.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findByIdSaldo(
    access_token: string,
    req: FindByIdSaldo,
  ): Promise<ApiResponseSaldo["data"]> {
    try {
      const response = await myApi.get(`/saldos/${req.id}`, {
        headers: { Authorization: `Bearer ${access_token}` },
      });

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findByActiveSaldo(
    access_token: string,
    req: FindAllSaldo,
  ): Promise<ApiResponsePaginationSaldoDeleteAt> {
    try {
      const response = await myApi.get("/saldos/active", {
        params: {
          page: req.page,
          page_size: req.page_size,
          search: req.search,
        },
        headers: { Authorization: `Bearer ${access_token}` },
      });

      if (response.status == 200) {
        return response.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findByCardNumberSaldo(
    access_token: string,
    req: FindByCardNumberSaldo,
  ): Promise<ApiResponseSaldo["data"]> {
    try {
      const response = await myApi.get(
        `/saldos/by-card/${req.cardNumber}`,
        {
          headers: { Authorization: `Bearer ${access_token}` },
        },
      );

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async createSaldo(
    access_token: string,
    req: CreateSaldo,
  ): Promise<ApiResponseSaldo["data"]> {
    try {
      const response = await myApi.post(
        "/saldos/create",
        {
          card_number: req.card_number,
          total_balance: req.total_balance,
        },
        {
          headers: { Authorization: `Bearer ${access_token}` },
        },
      );

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }
  async updateSaldo(
    access_token: string,
    req: UpdateSaldo,
  ): Promise<ApiResponseSaldo["data"]> {
    try {
      const response = await myApi.post(
        `/saldos/update/${req.id}`,
        {
          card_number: req.card_number,
          total_balance: req.total_balance,
        },
        {
          headers: { Authorization: `Bearer ${access_token}` },
        },
      );

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async trashedSaldo(
    access_token: string,
    req: TrashedSaldo,
  ): Promise<ApiResponseSaldo["data"]> {
    try {
      const response = await myApi.post(`/saldos/trash/${req.id}`, null, {
        headers: { Authorization: `Bearer ${access_token}` },
      });

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }
}

export default new SaldoService();
