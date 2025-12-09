import myApi from "@/helpers/api";
import {
    CreateCard,
    FindAllCard,
    FindByCardNumber,
    FindByIdCard,
    UpdateCard,
} from "@/types/domain/request";
import { FindByUser } from "@/types/domain/request/card/user";
import {
    ApiResponseCard,
    ApiResponseDashboardCard,
    ApiResponseDashboardCardNumber,
    ApiResponseMonthlyBalance,
    ApiResponseMonthlyTopupAmount,
    ApiResponseMonthlyTransactionAmount,
    ApiResponseMonthlyTransferAmount,
    ApiResponseMonthlyWithdrawAmount,
    ApiResponsePaginationCard,
    ApiResponsePaginationCardDeleteAt,
    ApiResponseYearlyBalance,
    ApiResponseYearlyTopupAmount,
    ApiResponseYearlyTransactionAmount,
    ApiResponseYearlyTransferAmount,
    ApiResponseYearlyWithdrawAmount,
} from "@/types/domain/response";

class CardService {
    async findDashboard(
        access_token: string,
    ): Promise<ApiResponseDashboardCard["data"]> {
        try {
            const response = await myApi.get("/cards/dashboard", {
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

    async findDashboardByCardNumber(
        access_token: string,
        card_number: string,
    ): Promise<ApiResponseDashboardCardNumber["data"]> {
        try {
            const response = await myApi.get("/cards/dashboard/" + card_number, {
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

    async findMonthBalance(
        access_token: string,
        year: number,
    ): Promise<ApiResponseMonthlyBalance["data"]> {
        try {
            const response = await myApi.get("/cards/stats/balance/monthly", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year },
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
    ): Promise<ApiResponseYearlyBalance["data"]> {
        try {
            const response = await myApi.get("/cards/stats/balance/yearly", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year },
            });
            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async findMonthTopupAmount(
        access_token: string,
        year: number,
    ): Promise<ApiResponseMonthlyTopupAmount["data"]> {
        try {
            const response = await myApi.get("/cards/stats/topup/monthly", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year },
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
    ): Promise<ApiResponseYearlyTopupAmount["data"]> {
        try {
            const response = await myApi.get("/cards/stats/topup/yearly", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year },
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
    ): Promise<ApiResponseMonthlyWithdrawAmount["data"]> {
        try {
            const response = await myApi.get("/cards/stats/withdraw/monthly", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year },
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
    ): Promise<ApiResponseYearlyWithdrawAmount["data"]> {
        try {
            const response = await myApi.get("/cards/stats/withdraw/yearly", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year },
            });
            if (response.status == 200) {
                return response.data.data;
            }
            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async findMonthlyTransferSenderAmount(
        access_token: string,
        year: number,
    ): Promise<ApiResponseMonthlyTransferAmount["data"]> {
        try {
            const response = await myApi.get("/cards/stats/transfer/monthly/sender", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year },
            });

            if (response.status == 200) {
                return response.data.data;
            }

            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async findYearlyTransferSenderAmount(
        access_token: string,
        year: number,
    ): Promise<ApiResponseYearlyTransferAmount["data"]> {
        try {
            const response = await myApi.get("/cards/stats/transfer/yearly/sender", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year },
            });
            if (response.status == 200) {
                return response.data.data;
            }
            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async findMonthlyTransferReceiverAmount(
        access_token: string,
        year: number,
    ): Promise<ApiResponseMonthlyTransferAmount["data"]> {
        try {
            const response = await myApi.get(
                "/cards/stats/transfer/monthly/receiver",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: { year },
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

    async findYearlyTransferReceiverAmount(
        access_token: string,
        year: number,
    ): Promise<ApiResponseYearlyTransferAmount["data"]> {
        try {
            const response = await myApi.get(
                "/cards/stats/transfer/yearly/receiver",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: { year },
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

    async findMonthlyTransactionAmount(
        access_token: string,
        year: number,
    ): Promise<ApiResponseMonthlyTransactionAmount["data"]> {
        try {
            const response = await myApi.get("/cards/stats/transaction/monthly", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year },
            });

            if (response.status == 200) {
                return response.data.data;
            }

            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async findYearlyTransactionAmount(
        access_token: string,
        year: number,
    ): Promise<ApiResponseYearlyTransferAmount["data"]> {
        try {
            const response = await myApi.get("/cards/stats/transaction/yearly", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year },
            });
            if (response.status == 200) {
                return response.data.data;
            }
            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async findMonthlyBalanceByCard(
        access_token: string,
        year: number,
        card_number: String,
    ): Promise<ApiResponseMonthlyBalance["data"]> {
        try {
            const response = await myApi.get("/cards/stats/balance/monthly/by-card", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year, card_number },
            });

            if (response.status == 200) {
                return response.data.data;
            }

            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async findYearlyBalanceByCard(
        access_token: string,
        year: number,
        card_number: String,
    ): Promise<ApiResponseYearlyBalance["data"]> {
        try {
            const response = await myApi.get("/cards/stats/balance/yearly/by-card", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year, card_number },
            });
            if (response.status == 200) {
                return response.data.data;
            }
            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async findMonthlyTopupAmountByCard(
        access_token: string,
        year: number,
        card_number: String,
    ): Promise<ApiResponseMonthlyTopupAmount["data"]> {
        try {
            const response = await myApi.get("/cards/stats/topup/monthly/by-card", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year, card_number },
            });

            if (response.status == 200) {
                return response.data.data;
            }

            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async findYearlyTopupAmountByCard(
        access_token: string,
        year: number,
        card_number: String,
    ): Promise<ApiResponseYearlyTopupAmount["data"]> {
        try {
            const response = await myApi.get("/cards/stats/topup/yearly/by-card", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year, card_number },
            });
            if (response.status == 200) {
                return response.data.data;
            }
            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async findMonthlyWithdrawAmountByCard(
        access_token: string,
        year: number,
        card_number: String,
    ): Promise<ApiResponseMonthlyWithdrawAmount["data"]> {
        try {
            const response = await myApi.get(
                "/cards/stats/withdraw/monthly/by-card",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: { year, card_number },
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

    async findYearlyWithdrawAmountByCard(
        access_token: string,
        year: number,
        card_number: String,
    ): Promise<ApiResponseYearlyWithdrawAmount["data"]> {
        try {
            const response = await myApi.get("/cards/stats/withdraw/yearly/by-card", {
                headers: { Authorization: `Bearer ${access_token}` },
                params: { year, card_number },
            });
            if (response.status == 200) {
                return response.data.data;
            }
            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async findMonthlyTransactionAmountByCard(
        access_token: string,
        year: number,
        card_number: String,
    ): Promise<ApiResponseMonthlyTransactionAmount["data"]> {
        try {
            const response = await myApi.get(
                "/cards/stats/transaction/monthly/by-card",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: { year, card_number },
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

    async findYearlyTransactionAmountByCard(
        access_token: string,
        year: number,
        card_number: String,
    ): Promise<ApiResponseYearlyTransactionAmount["data"]> {
        try {
            const response = await myApi.get(
                "/cards/stats/transaction/yearly/by-card",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: { year, card_number },
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

    async findMonthlyTransferSenderAmountByCard(
        access_token: string,
        year: number,
        card_number: String,
    ): Promise<ApiResponseMonthlyTransferAmount["data"]> {
        try {
            const response = await myApi.get(
                "/cards/stats/transfer/monthly/by-card/sender",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: { year, card_number },
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

    async findYearlyTransferSenderAmountByCard(
        access_token: string,
        year: number,
        card_number: String,
    ): Promise<ApiResponseYearlyTransferAmount["data"]> {
        try {
            const response = await myApi.get(
                "/cards/stats/transfer/yearly/by-card/sender",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: { year, card_number },
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

    async findMonthlyTransferReceiverAmountByCard(
        access_token: string,
        year: number,
        card_number: String,
    ): Promise<ApiResponseMonthlyTransferAmount["data"]> {
        try {
            const response = await myApi.get(
                "/cards/stats/transfer/monthly/by-card/receiver",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: { year, card_number },
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

    async findYearlyTransferReceiverAmountByCard(
        access_token: string,
        year: number,
        card_number: String,
    ): Promise<ApiResponseYearlyTransferAmount["data"]> {
        try {
            const response = await myApi.get(
                "/cards/stats/transfer/yearly/by-card/receiver",
                {
                    headers: { Authorization: `Bearer ${access_token}` },
                    params: { year, card_number },
                },
            );
            console.log(response.data.data);
            if (response.status == 200) {
                return response.data.data;
            }
            throw new Error(response.data.message || "Login failed.");
        } catch (error: any) {
            throw new Error(error.response?.data?.message || "Login failed.");
        }
    }

    async findAllCards(
        req: FindAllCard,
        access_token: string,
    ): Promise<ApiResponsePaginationCard> {
        try {
            const response = await myApi.get("/cards", {
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

    async findByIdCard(
        req: FindByIdCard,
        access_token: string,
    ): Promise<ApiResponseCard["data"]> {
        try {
            const response = await myApi.get(`/cards/${req.id}`, {
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

    async findByUser(
        req: FindByUser,
        access_token: string,
    ): Promise<ApiResponseCard["data"]> {
        try {
            const response = await myApi.get(`/cards/by-user/${req.id}`, {
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

    async findByCardNumber(
        req: FindByCardNumber,
        access_token: string,
    ): Promise<ApiResponseCard["data"]> {
        try {
            const response = await myApi.get(`/cards/by-card/${req.cardNumber}`, {
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

    async findByActiveCard(
        req: FindAllCard,
        access_token: string,
    ): Promise<ApiResponsePaginationCardDeleteAt> {
        try {
            const response = await myApi.get("/cards/active", {
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

    async createCard(
        req: CreateCard,
        access_token: string,
    ): Promise<ApiResponseCard["data"]> {
        try {
            const response = await myApi.post(
                "/cards/create",
                {
                    user_id: req.user_id,
                    card_type: req.card_type,
                    expire_date: req.expire_date,
                    cvv: req.cvv,
                    card_provider: req.card_provider,
                },
                {
                    headers: { Authorization: `Bearer ${access_token}` },
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

    async updateCard(
        req: UpdateCard,
        access_token: string,
    ): Promise<ApiResponseCard["data"]> {
        try {
            const response = await myApi.post(
                `/cards/update/${req.card_id}`,
                {
                    card_id: req.card_id,
                    user_id: req.user_id,
                    card_type: req.card_type,
                    expire_date: req.expire_date,
                    cvv: req.cvv,
                    card_provider: req.card_provider,
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

    async trashedCard(
        req: FindByIdCard,
        access_token: string,
    ): Promise<ApiResponseCard["data"]> {
        try {
            const response = await myApi.post(`/cards/trash/${req.id}`, null, {
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

export default new CardService();
