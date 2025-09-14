export interface Transfer {
  id: number;
  transfer_from: string;
  transfer_to: string;
  transfer_amount: number;
  transfer_time: string;
  created_at: string;
  updated_at: string;
}

export interface TransferTrashed {
  id: number;
  transfer_from: string;
  transfer_to: string;
  transfer_amount: number;
  transfer_time: string;
  created_at: string;
  updated_at: string;
  deleted_at: string;
}

export interface TransferMonthStatusSuccess {
  year: string;
  month: string;
  total_amount: number;
  total_success: number;
}

export interface TransferYearStatusSuccess {
  year: string;
  total_amount: number;
  total_success: number;
}

export interface TransferMonthStatusFailed {
  year: string;
  month: string;
  total_amount: number;
  total_failed: number;
}

export interface TransferYearStatusFailed {
  year: string;
  total_amount: number;
  total_failed: number;
}

export interface TransferMonthAmount {
  month: string;
  total_amount: number;
}

export interface TransferYearAmount {
  year: string;
  total_amount: number;
}

