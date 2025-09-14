import myApi from "@/helpers/api";
import {
  CreateTopup,
  FindAllTopup,
  FindByCardNumberTopup,
  FindByIdTopup,
  TrashedTopup,
  UpdateTopup,
} from "@/types/domain/request/topup";
import {
  ApiResponsePaginationTopup,
  ApiResponsePaginationTopupDeleteAt,
  ApiResponseTopup,
  ApiResponseTopupMonthAmount,
  ApiResponseTopupMonthMethod,
  ApiResponseTopupMonthStatusFailed,
  ApiResponseTopupMonthStatusSuccess,
  ApiResponseTopupYearAmount,
  ApiResponseTopupYearMethod,
  ApiResponseTopupYearStatusFailed,
  ApiResponseTopupYearStatusSuccess,
} from "@/types/domain/response";

class TopupService {
  async findMonthStatusSuccess(
    access_token: string,
    year: number,
    month: number,
  ): Promise<ApiResponseTopupMonthStatusSuccess["data"]> {
    try {
      const response = await myApi.get("/topups/stats/status/success/monthly", {
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
  ): Promise<ApiResponseTopupYearStatusSuccess["data"]> {
    try {
      const response = await myApi.get("/topups/stats/status/success/yearly", {
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
  ): Promise<ApiResponseTopupMonthStatusFailed["data"]> {
    try {
      const response = await myApi.get("/topups/stats/status/failed/monthly", {
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
  ): Promise<ApiResponseTopupYearStatusFailed["data"]> {
    try {
      const response = await myApi.get("/topups/stats/status/failed/yearly", {
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
  ): Promise<ApiResponseTopupMonthStatusSuccess["data"]> {
    try {
      const response = await myApi.get("/topups/stats/status/success/monthly/by-card", {
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
  ): Promise<ApiResponseTopupYearStatusSuccess["data"]> {
    try {
      const response = await myApi.get("/topups/stats/status/success/yearly/by-card", {
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
  ): Promise<ApiResponseTopupMonthStatusFailed["data"]> {
    try {
      const response = await myApi.get("/topups/stats/status/failed/monthly/by-card", {
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
  ): Promise<ApiResponseTopupYearStatusFailed["data"]> {
    try {
      const response = await myApi.get("/topups/stats/status/failed/yearly/by-card", {
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

  async findMonthTopupMethod(
    access_token: string,
    year: number,
  ): Promise<ApiResponseTopupMonthMethod["data"]> {
    try {
      const response = await myApi.get("/topups/stats/method/monthly", {
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

  async findYearTopupMethod(
    access_token: string,
    year: number,
  ): Promise<ApiResponseTopupYearMethod["data"]> {
    try {
      const response = await myApi.get("/topups/stats/method/yearly", {
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

  async findMonthTopupAmount(
    access_token: string,
    year: number,
  ): Promise<ApiResponseTopupMonthAmount["data"]> {
    try {
      const response = await myApi.get("/topups/stats/amount/monthly", {
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

  async findYearTopupAmount(
    access_token: string,
    year: number,
  ): Promise<ApiResponseTopupYearAmount["data"]> {
    try {
      const response = await myApi.get("/topups/stats/amount/yearly", {
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

  async findMonthTopupMethodCard(
    access_token: string,
    year: number,
    card_number: string,
  ): Promise<ApiResponseTopupMonthMethod["data"]> {
    try {
      const response = await myApi.get("/topups/stats/method/monthly/by-card", {
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

  async findYearTopupMethodCard(
    access_token: string,
    year: number,
    card_number: string,
  ): Promise<ApiResponseTopupYearMethod["data"]> {
    try {
      const response = await myApi.get("/topups/stats/method/yearlys/by-card", {
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

  async findMonthTopupAmountCard(
    access_token: string,
    year: number,
    card_number: string,
  ): Promise<ApiResponseTopupMonthAmount["data"]> {
    try {
      const response = await myApi.get("/topups/stats/amount/monthly/by-card", {
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

  async findYearTopupAmountCard(
    access_token: string,
    year: number,
    card_number: string,
  ): Promise<ApiResponseTopupYearAmount["data"]> {
    try {
      const response = await myApi.get("/topups/stats/amount/yearly/by-card", {
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

  async findAllTopups(
    access_token: string,
    req: FindAllTopup,
  ): Promise<ApiResponsePaginationTopup> {
    try {
      const response = await myApi.get("/topups", {
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

  async findByIdTopup(
    access_token: string,
    req: FindByIdTopup,
  ): Promise<ApiResponseTopup["data"]> {
    try {
      const response = await myApi.get(`/topups/${req.id}`, {
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

  async findByActiveTopup(
    access_token: string,
    req: FindAllTopup,
  ): Promise<ApiResponsePaginationTopupDeleteAt> {
    try {
      const response = await myApi.get("/topups/active", {
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

  async findByCardNumberTopup(
    access_token: string,
    req: FindByCardNumberTopup,
  ): Promise<ApiResponsePaginationTopup> {
    try {
      const response = await myApi.get(
        `/topups/by-card/${req.cardNumber}`,
        {
          params: {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
          },
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

  async createTopup(
    access_token: string,
    req: CreateTopup,
  ): Promise<ApiResponseTopup["data"]> {
    try {
      const response = await myApi.post(
        "/topups/create",
        {
          card_number: req.card_number,
          topup_amount: req.topup_amount,
          topup_method: req.topup_method,
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
  async updateTopup(
    access_token: string,
    req: UpdateTopup,
  ): Promise<ApiResponseTopup["data"]> {
    try {
      const response = await myApi.post(
        `/topups/update/${req.id}`,
        {
          card_number: req.card_number,
          topup_amount: req.topup_amount,
          topup_method: req.topup_method,
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

  async trashedTopup(
    access_token: string,
    req: TrashedTopup,
  ): Promise<ApiResponseTopup["data"]> {
    try {
      const response = await myApi.post(`/topups/trash/${req.id}`, null, {
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

export default new TopupService();
