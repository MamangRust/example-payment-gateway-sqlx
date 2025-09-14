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
import { invoke } from "@tauri-apps/api/core";

class TransferCommand {
  async findMonthStatusSuccessTransfer(
    accessToken: string,
    year: number,
    month: number,
  ): Promise<ApiResponseTransferMonthStatusSuccess> {
    try {
      return await invoke("find_month_status_success_transfer", {
        accessToken,
        year,
        month,
      });
    } catch (error) {
      console.error("Error in findMonthStatusSuccessTransfer:", error);
      throw error;
    }
  }

  async findYearStatusSuccessTransfer(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTransferYearStatusSuccess> {
    try {
      return await invoke("find_year_status_success_transfer", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearStatusSuccessTransfer:", error);
      throw error;
    }
  }

  async findMonthStatusFailedTransfer(
    accessToken: string,
    year: number,
    month: number,
  ): Promise<ApiResponseTransferMonthStatusFailed> {
    try {
      return await invoke("find_month_status_failed_transfer", {
        accessToken,
        year,
        month,
      });
    } catch (error) {
      console.error("Error in findMonthStatusFailedTransfer:", error);
      throw error;
    }
  }

  async findYearStatusFailedTransfer(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTransferYearStatusFailed> {
    try {
      return await invoke("find_year_status_failed_transfer", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearStatusFailedTransfer:", error);
      throw error;
    }
  }

  async findMonthStatusSuccessTransferByCardNumber(
    accessToken: string,
    year: number,
    month: number,
    cardNumber: string,
  ): Promise<ApiResponseTransferMonthStatusSuccess> {
    try {
      return await invoke("find_month_status_success_transfer_by_card_number", {
        accessToken,
        year,
        month,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthStatusSuccessTransfer:", error);
      throw error;
    }
  }

  async findYearStatusSuccessTransferByCardNumber(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTransferYearStatusSuccess> {
    try {
      return await invoke("find_year_status_success_transfer_by_card_number", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearStatusSuccessTransfer:", error);
      throw error;
    }
  }

  async findMonthStatusFailedTransferByCardNumber(
    accessToken: string,
    year: number,
    month: number,
    cardNumber: string,
  ): Promise<ApiResponseTransferMonthStatusFailed> {
    try {
      return await invoke("find_month_status_failed_transfer_by_card_number", {
        accessToken,
        year,
        month,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthStatusFailedTransfer:", error);
      throw error;
    }
  }

  async findYearStatusFailedTransferByCardNumber(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTransferYearStatusFailed> {
    try {
      return await invoke("find_year_status_failed_transfer_by_card_number", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearStatusFailedTransfer:", error);
      throw error;
    }
  }

  async findMonthTransferAmountTransfer(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTransferMonthAmount> {
    try {
      return await invoke("find_month_transfer_amount_transfer", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthTransferAmountTransfer:", error);
      throw error;
    }
  }

  async findYearTransferAmountTransfer(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTransferYearAmount> {
    try {
      return await invoke("find_year_transfer_amount_transfer", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearTransferAmountTransfer:", error);
      throw error;
    }
  }

  async findMonthTransferAmountBySender(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTransferMonthAmount> {
    try {
      return await invoke("find_month_transfer_amount_by_sender", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthTransferAmountBySender:", error);
      throw error;
    }
  }

  async findYearTransferAmountBySender(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTransferYearAmount> {
    try {
      return await invoke("find_year_transfer_amount_by_sender", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearTransferAmountBySender:", error);
      throw error;
    }
  }

  async findMonthTransferAmountByReceiver(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTransferMonthAmount> {
    try {
      return await invoke("find_month_transfer_amount_by_receiver", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthTransferAmountByReceiver:", error);
      throw error;
    }
  }

  async findYearTransferAmountByReceiver(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTransferYearAmount> {
    try {
      return await invoke("find_year_transfer_amount_by_receiver", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearTransferAmountByReceiver:", error);
      throw error;
    }
  }

  async findAllTransfers(
    accessToken: string,
    req: FindAllTransfer,
  ): Promise<ApiResponsePaginationTransfer> {
    try {
      return await invoke("find_all_transfers", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findAllTransfers:", error);
      throw error;
    }
  }

  async findByIdTransfer(
    accessToken: string,
    req: FindByIdTransfer,
  ): Promise<ApiResponseTransfer> {
    try {
      return await invoke("find_by_id_transfer", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByIdTransfer:", error);
      throw error;
    }
  }

  async findByTransferFrom(
    accessToken: string,
    req: TransferFrom,
  ): Promise<ApiResponseTransfers> {
    try {
      return await invoke("find_by_transfer_from", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByTransferFrom:", error);
      throw error;
    }
  }

  async findByTransferTo(
    accessToken: string,
    req: TransferTo,
  ): Promise<ApiResponseTransfers> {
    try {
      return await invoke("find_by_transfer_to", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByTransferTo:", error);
      throw error;
    }
  }

  async findByActiveTransfer(
    accessToken: string,
    req: FindAllTransfer,
  ): Promise<ApiResponsePaginationTransferDeleteAt> {
    try {
      return await invoke("find_by_active_transfer", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByActiveTransfer:", error);
      throw error;
    }
  }

  async createTransfer(
    accessToken: string,
    req: CreateTransfer,
  ): Promise<ApiResponseTransfer> {
    try {
      return await invoke("create_transfer", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in createTransfer:", error);
      throw error;
    }
  }

  async updateTransfer(
    accessToken: string,
    req: UpdateTransfer,
  ): Promise<ApiResponseTransfer> {
    try {
      return await invoke("update_transfer", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in updateTransfer:", error);
      throw error;
    }
  }

  async trashedTransfer(
    accessToken: string,
    req: TrashedTransfer,
  ): Promise<ApiResponseTransfer> {
    try {
      return await invoke("trashed_transfer", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in trashedTransfer:", error);
      throw error;
    }
  }
}

export default new TransferCommand();
