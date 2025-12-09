export interface Withdraw {
  withdraw_id: number;
  card_number: string;
  withdraw_amount: number;
  withdraw_time: Date;
  created_at: string;
  updated_at: string;
}

export interface WithdrawTrashed {
  withdraw_id: number;
  card_number: string;
  withdraw_amount: number;
  withdraw_time: Date;
  created_at: string;
  updated_at: string;
  deleted_at: string;
}

export interface WithdrawMonthStatusSuccess {
  year: string;
  month: string;
  total_amount: number;
  total_success: number;
}

export interface WithdrawYearStatusSuccess {
  year: string;
  total_amount: number;
  total_success: number;
}

export interface WithdrawMonthStatusFailed {
  year: string;
  month: string;
  total_amount: number;
  total_failed: number;
}

export interface WithdrawYearStatusFailed {
  year: string;
  total_amount: number;
  total_failed: number;
}

export interface WithdrawMonthlyAmount {
  month: string;
  total_amount: number;
}

export interface WithdrawYearlyAmount {
  year: string;
  total_amount: number;
}
