import myApi from "@/helpers/api";
import {
  CreateWithdraw,
  FindByCardNumberWithdraw,
  FindByIdWithdraw,
  TrashedWithdraw,
  UpdateWithdraw,
} from "@/types/domain/request";
import { FindAllWithdraw } from "@/types/domain/request/withdraw/list";
import {
  ApiResponsePaginationWithdraw,
  ApiResponsePaginationWithdrawDeleteAt,
  ApiResponseWithdraw,
  ApiResponseWithdrawMonthAmount,
  ApiResponseWithdrawMonthStatusFailed,
  ApiResponseWithdrawMonthStatusSuccess,
  ApiResponseWithdrawYearAmount,
  ApiResponseWithdrawYearStatusFailed,
  ApiResponseWithdrawYearStatusSuccess,
} from "@/types/domain/response";

class WithdrawService {
  async findMonthStatusSuccess(
    access_token: string,
    year: number,
    month: number,
  ): Promise<ApiResponseWithdrawMonthStatusSuccess["data"]> {
    try {
      const response = await myApi.get("/withdraws/stats/status/success/monthly", {
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

  async findYearStatusSuccess(
    access_token: string,
    year: number,
  ): Promise<ApiResponseWithdrawYearStatusSuccess["data"]> {
    try {
      const response = await myApi.get("/withdraws/stats/status/success/yearly", {
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

  async findMonthStatusFailed(
    access_token: string,
    year: number,
    month: number,
  ): Promise<ApiResponseWithdrawMonthStatusFailed["data"]> {
    try {
      const response = await myApi.get("/withdraws/stats/status/failed/monthly", {
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

  async findYearStatusFailed(
    access_token: string,
    year: number,
  ): Promise<ApiResponseWithdrawYearStatusFailed["data"]> {
    try {
      const response = await myApi.get("/withdraws/stats/status/failed/yearly", {
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

  async findMonthStatusSuccessByCardNumber(
    access_token: string,
    year: number,
    month: number,
    cardNumber: string,
  ): Promise<ApiResponseWithdrawMonthStatusSuccess["data"]> {
    try {
      const response = await myApi.get("/withdraws/stats/status/success/monthly/by-card", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
          month,
          card_number: cardNumber,
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

  async findYearStatusSuccessByCardNumber(
    access_token: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseWithdrawYearStatusSuccess["data"]> {
    try {
      const response = await myApi.get("/withdraws/stats/status/success/yearly/by-card", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
          card_number: cardNumber,
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

  async findMonthStatusFailedByCardNumber(
    access_token: string,
    year: number,
    month: number,
    cardNumber: string,
  ): Promise<ApiResponseWithdrawMonthStatusFailed["data"]> {
    try {
      const response = await myApi.get("/withdraws/stats/status/failed/monthly/by-card", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
          month,
          card_number: cardNumber,
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

  async findYearStatusFailedByCardNumber(
    access_token: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseWithdrawYearStatusFailed["data"]> {
    try {
      const response = await myApi.get("/withdraws/stats/status/failed/yearly/by-card", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
          card_number: cardNumber,
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

  async findMonthWithdrawAmount(
    access_token: string,
    year: number,
  ): Promise<ApiResponseWithdrawMonthAmount["data"]> {
    try {
      const response = await myApi.get("/withdraws/stats/amount/monthly", {
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

  async findYearWithdrawAmount(
    access_token: string,
    year: number,
  ): Promise<ApiResponseWithdrawYearAmount["data"]> {
    try {
      const response = await myApi.get("/withdraws/stats/amount/yearly", {
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

  async findMonthWithdrawAmountByCard(
    access_token: string,
    year: number,
    card_number: string,
  ): Promise<ApiResponseWithdrawMonthAmount["data"]> {
    try {
      const response = await myApi.get("/withdraws/stats/amount/monthly/by-card", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
          card_number,
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

  async findYearWithdrawAmountByCard(
    access_token: string,
    year: number,
    card_number: string,
  ): Promise<ApiResponseWithdrawYearAmount["data"]> {
    try {
      const response = await myApi.get("/withdraws/stats/amount/yearly/by-card", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
          card_number,
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

  async findAllWithdraws(
    access_token: string,
    req: FindAllWithdraw,
  ): Promise<ApiResponsePaginationWithdraw> {
    try {
      const response = await myApi.get("/withdraws", {
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
  async findByIdWithdraw(
    access_token: string,
    req: FindByIdWithdraw,
  ): Promise<ApiResponseWithdraw["data"]> {
    try {
      const response = await myApi.get(`/withdraws/${req.id}`, {
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

  async findAllByCardNumberWithdraw(
    access_token: string,
    req: FindByCardNumberWithdraw,
  ): Promise<ApiResponsePaginationWithdraw> {
    try {
      const response = await myApi.get(
        `/withdraws/by-card/${req.cardNumber}`,
        {
          headers: { Authorization: `Bearer ${access_token}` },
        },
      );

      if (response.status == 200) {
        return response.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findByActiveWithdraw(
    access_token: string,
    req: FindAllWithdraw,
  ): Promise<ApiResponsePaginationWithdrawDeleteAt> {
    try {
      const response = await myApi.get("/withdraws/active", {
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

  async createWithdraw(
    access_token: string,
    req: CreateWithdraw,
  ): Promise<ApiResponseWithdraw["data"]> {
    try {
      const response = await myApi.post(
        "/withdraws/create",
        {
          card_number: req.card_number,
          withdraw_amount: req.withdraw_amount,
          withdraw_time: req.withdraw_time,
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

  async updateWithdraw(
    access_token: string,
    req: UpdateWithdraw,
  ): Promise<ApiResponseWithdraw["data"]> {
    try {
      const response = await myApi.post(
        `/withdraws/update/${req.id}`,
        {
          withdraw_id: req.id,
          card_number: req.card_number,
          withdraw_amount: req.withdraw_amount,
          withdraw_time: req.withdraw_time,
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
  async trashedWithdraw(
    access_token: string,
    req: TrashedWithdraw,
  ): Promise<ApiResponseWithdraw["data"]> {
    try {
      const response = await myApi.post(`/withdraws/trash/${req.id}`, null, {
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

export default new WithdrawService();
