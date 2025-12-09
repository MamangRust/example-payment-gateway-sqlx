import myApi from "@/helpers/api";
import {
    CreateTransfer,
    FindAllTransfer,
    FindByIdTransfer,
    TransferFrom,
    TransferTo,
    TrashedTransfer,
    UpdateTransfer,
} from "@/types/domain/request";
import {
    ApiResponsePaginationTransfer,
    ApiResponsePaginationTransferDeleteAt,
    ApiResponseTransfer,
    ApiResponseTransferMonthAmount,
    ApiResponseTransfers,
    ApiResponseTransferMonthStatusSuccess,
    ApiResponseTransferYearStatusSuccess,
    ApiResponseTransferMonthStatusFailed,
    ApiResponseTransferYearStatusFailed,
    ApiResponseTransferYearAmount,
} from "@/types/domain/response";

class TranferService {
    async findMonthStatusSuccess(
        access_token: string,
        year: number,
        month: number,
    ): Promise<ApiResponseTransferMonthStatusSuccess["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/status/success/monthly", {
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
    ): Promise<ApiResponseTransferYearStatusSuccess["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/status/success/yearly", {
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
    ): Promise<ApiResponseTransferMonthStatusFailed["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/status/failed/monthly", {
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
    ): Promise<ApiResponseTransferYearStatusFailed["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/status/failed/yearly", {
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
    ): Promise<ApiResponseTransferMonthStatusSuccess["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/status/success/monthly/by-card", {
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
    ): Promise<ApiResponseTransferYearStatusSuccess["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/status/success/yearly/by-card", {
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
    ): Promise<ApiResponseTransferMonthStatusFailed["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/status/failed/monthly/by-card", {
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
    ): Promise<ApiResponseTransferYearStatusFailed["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/status/failed/yearly/by-card", {
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

    async findMonthTransferAmount(
        access_token: string,
        year: number,
    ): Promise<ApiResponseTransferMonthAmount["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/amount/monthly", {
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
    async findYearTransferAmount(
        access_token: string,
        year: number,
    ): Promise<ApiResponseTransferYearAmount["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/amount/yearly", {
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

    async findMonthTransferAmountBySender(
        access_token: string,
        year: number,
        card_number: string,
    ): Promise<ApiResponseTransferMonthAmount["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/amount/monthly/sender", {
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
    async findYearTransferAmountBySender(
        access_token: string,
        year: number,
        card_number: string,
    ): Promise<ApiResponseTransferYearAmount["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/amount/yearly/sender", {
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

    async findMonthTransferAmountByReceiver(
        access_token: string,
        year: number,
        card_number: string,
    ): Promise<ApiResponseTransferMonthAmount["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/amount/monthly/receiver", {
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
    async findYearTransferAmountByReceiver(
        access_token: string,
        year: number,
        card_number: string,
    ): Promise<ApiResponseTransferYearAmount["data"]> {
        try {
            const response = await myApi.get("/transfers/stats/amount/yearly/receiver", {
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
    async findAllTransfers(
        access_token: string,
        req: FindAllTransfer,
    ): Promise<ApiResponsePaginationTransfer> {
        try {
            const response = await myApi.get("/transfers", {
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
    async findByIdTransfer(
        access_token: string,
        req: FindByIdTransfer,
    ): Promise<ApiResponseTransfer> {
        try {
            const response = await myApi.get(`/transfers/${req.id}`, {
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

    async findByTransferFrom(
        access_token: string,
        req: TransferFrom,
    ): Promise<ApiResponseTransfers["data"]> {
        try {
            const response = await myApi.get(`/transfers/from/${req.cardNumber}`, {
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

    async findByTransferTo(
        access_token: string,
        req: TransferTo,
    ): Promise<ApiResponseTransfers["data"]> {
        try {
            const response = await myApi.get(`/transfers/to/${req.cardNumber}`, {
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

    async findByActiveTransfer(
        access_token: string,
        req: FindAllTransfer,
    ): Promise<ApiResponsePaginationTransferDeleteAt> {
        try {
            const response = await myApi.get("/transfers/active", {
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

    async createTransfer(
        access_token: string,
        req: CreateTransfer,
    ): Promise<ApiResponseTransfer["data"]> {
        try {
            const response = await myApi.post(
                "/transfers/create",
                {
                    transfer_from: req.transfer_from,
                    transfer_to: req.transfer_to,
                    transfer_amount: req.transfer_amount,
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
    async updateTransfer(
        access_token: string,
        req: UpdateTransfer,
    ): Promise<ApiResponseTransfer["data"]> {
        try {
            const response = await myApi.post(
                "/transfers/update",
                {
                    transfer_from: req.transfer_from,
                    transfer_to: req.transfer_to,
                    transfer_amount: req.transfer_amount,
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
    async trashedTransfer(
        access_token: string,
        req: TrashedTransfer,
    ): Promise<ApiResponseTransfer["data"]> {
        try {
            const response = await myApi.post(`/transfers/trash/${req.id}`, null, {
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

export default new TranferService();
