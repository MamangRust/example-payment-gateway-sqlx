import myApi from "@/helpers/api";
import {
    CreateTransaction,
    FindAllTransaction,
    FindyByCardNumberTransaction,
    FindyByIdTransaction,
    FindyByMerchantTransaction,
    TrashedTransaction,
    UpdateTransaction,
} from "@/types/domain/request";
import {
    ApiResponsePaginationTransaction,
    ApiResponsePaginationTransactionDeleteAt,
    ApiResponseTransaction,
    ApiResponseTransactionMonthAmount,
    ApiResponseTransactionMonthMethod,
    ApiResponseTransactionMonthStatusFailed,
    ApiResponseTransactionMonthStatusSuccess,
    ApiResponseTransactions,
    ApiResponseTransactionYearAmount,
    ApiResponseTransactionYearMethod,
    ApiResponseTransactionYearStatusFailed,
    ApiResponseTransactionYearStatusSuccess,
} from "@/types/domain/response";

class TransactionService {
    async findMonthStatusSuccess(
        access_token: string,
        year: number,
        month: number,
    ): Promise<ApiResponseTransactionMonthStatusSuccess["data"]> {
        try {
            const response = await myApi.get("/transactions/stats/status/success/monthly", {
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
    ): Promise<ApiResponseTransactionYearStatusSuccess["data"]> {
        try {
            const response = await myApi.get("/transactions/stats/status/success/yearly", {
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
    ): Promise<ApiResponseTransactionMonthStatusFailed["data"]> {
        try {
            const response = await myApi.get("/transactions/stats/status/failed/monthly", {
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
    ): Promise<ApiResponseTransactionYearStatusFailed["data"]> {
        try {
            const response = await myApi.get("/transactions/stats/status/failed/yearly", {
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
    ): Promise<ApiResponseTransactionMonthStatusSuccess["data"]> {
        try {
            const response = await myApi.get(
                "/transactions/stats/status/success/monthly/by-card",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: {
                        year,
                        month,
                        card_number: cardNumber,
                    },
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

    async findYearStatusSuccessByCardNumber(
        access_token: string,
        year: number,
        cardNumber: string,
    ): Promise<ApiResponseTransactionYearStatusSuccess["data"]> {
        try {
            const response = await myApi.get("/transactions/stats/status/success/yearly/by-card", {
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
    ): Promise<ApiResponseTransactionMonthStatusFailed["data"]> {
        try {
            const response = await myApi.get("/transactions/stats/status/failed/monthly/by-card", {
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
    ): Promise<ApiResponseTransactionYearStatusFailed["data"]> {
        try {
            const response = await myApi.get("/transactions/stats/status/failed/yearly/by-card", {
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

    async findMonthTransactionMethod(
        access_token: string,
        year: number,
    ): Promise<ApiResponseTransactionMonthMethod["data"]> {
        try {
            const response = await myApi.get(
                "/transactions/stats/method/monthly",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: {
                        year,
                    },
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

    async findYearTransactionMethod(
        access_token: string,
        year: number,
    ): Promise<ApiResponseTransactionYearMethod["data"]> {
        try {
            const response = await myApi.get("/transactions/stats/method/yearly", {
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

    async findMonthTransactionAmount(
        access_token: string,
        year: number,
    ): Promise<ApiResponseTransactionMonthAmount["data"]> {
        try {
            const response = await myApi.get("/transactions/stats/amount/monthly", {
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

    async findYearTransactionAmount(
        access_token: string,
        year: number,
    ): Promise<ApiResponseTransactionYearAmount["data"]> {
        try {
            const response = await myApi.get("/transactions/stats/amount/yearly", {
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

    async findMonthTransactionMethodCard(
        access_token: string,
        year: number,
        card_number: string,
    ): Promise<ApiResponseTransactionMonthMethod["data"]> {
        try {
            const response = await myApi.get(
                "/transactions/stats/method/monthly/by-card",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: {
                        year,
                        card_number,
                    },
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

    async findYearTransactionMethodCard(
        access_token: string,
        year: number,
        card_number: string,
    ): Promise<ApiResponseTransactionYearMethod["data"]> {
        try {
            const response = await myApi.get("/transactions/stats/method/yearly/by-card", {
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

    async findMonthTransactionAmountCard(
        access_token: string,
        year: number,
        card_number: string,
    ): Promise<ApiResponseTransactionMonthAmount["data"]> {
        try {
            const response = await myApi.get(
                "/transactions/stats/amount/monthly/by-card",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: {
                        year,
                        card_number,
                    },
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

    async findYearTransactionAmountCard(
        access_token: string,
        year: number,
        card_number: string,
    ): Promise<ApiResponseTransactionYearAmount["data"]> {
        try {
            const response = await myApi.get("/transactions/stats/amount/yearly/by-card", {
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

    async findAllTransactions(
        access_token: string,
        req: FindAllTransaction,
    ): Promise<ApiResponsePaginationTransaction> {
        try {
            const response = await myApi.get("/transactions", {
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
    async findByIdTransaction(
        access_token: string,
        req: FindyByIdTransaction,
    ): Promise<ApiResponseTransaction["data"]> {
        try {
            const response = await myApi.get(`/transactions/${req.id}`, {
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

    async findByCardNumberTransaction(
        access_token: string,
        req: FindyByCardNumberTransaction,
    ): Promise<ApiResponsePaginationTransaction> {
        try {
            const response = await myApi.get(
                `/transactions/by-card`,
                {
                    params: {
                        card_number: req.cardNumber,
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
    async findByMerchantTransaction(
        access_token: string,
        req: FindyByMerchantTransaction,
    ): Promise<ApiResponseTransactions["data"]> {
        try {
            const response = await myApi.get(`/transactions/merchant/${req.id}`, {
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

    async findByActiveTransaction(
        access_token: string,
        req: FindAllTransaction,
    ): Promise<ApiResponsePaginationTransactionDeleteAt> {
        try {
            const response = await myApi.get("/transactions/active", {
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

    async createTransaction(
        access_token: string,
        req: CreateTransaction,
    ): Promise<ApiResponseTransaction> {
        try {
            const response = await myApi.post(
                "/transactions/create",
                {
                    card_number: req.card_number,
                    amount: req.amount,
                    merchant_id: req.merchant_id,
                    payment_method: req.payment_method,
                    transaction_time: req.transaction_time,
                },
                {
                    headers: {
                        Authorization: `Bearer ${access_token}`,
                        "x-api-key": req.api_key,
                    },
                },
            );

            if (response.status == 201) {
                return response.data.data;
            }
            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async updateTransaction(
        access_token: string,
        req: UpdateTransaction,
    ): Promise<ApiResponseTransaction> {
        try {
            const response = await myApi.post(
                `/transactions/update/${req.id}`,
                {
                    transaction_id: req.id,
                    card_number: req.card_number,
                    amount: req.amount,
                    merchant_id: req.merchant_id,
                    payment_method: req.payment_method,
                    transaction_time: req.transaction_time,
                },
                {
                    headers: {
                        Authorization: `Bearer ${access_token}`,
                        "x-api-key": req.api_key,
                    },
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
    async trashedTransaction(
        access_token: string,
        req: TrashedTransaction,
    ): Promise<ApiResponseTransaction> {
        try {
            const response = await myApi.post(
                `/transactions/trash/${req.id}`,
                null,
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
}

export default new TransactionService();
