export interface Topup {
  id: number;
  card_number: string;
  topup_no: string;
  topup_method: string;
  topup_amount: string;
  topup_time: string;
  created_at: string;
  updated_at: string;
}

export interface TopupTrashed {
  id: number;
  card_number: string;
  topup_no: string;
  topup_method: string;
  topup_amount: string;
  topup_time: string;
  created_at: string;
  updated_at: string;
  deleted_at: string;
}

export interface TopupMonthStatusSuccess {
  year: string;
  month: string;
  total_amount: number;
  total_success: number;
}

export interface TopupYearStatusSuccess {
  year: string;
  total_amount: number;
  total_success: number;
}

export interface TopupMonthStatusFailed {
  year: string;
  total_amount: number;
  month: string;
  total_failed: number;
}

export interface TopupYearStatusFailed {
  year: string;
  total_amount: number;
  total_failed: number;
}

export interface TopupMonthMethod {
  month: string;
  topup_method: string;
  total_topups: number;
  total_amount: number;
}

export interface TopupYearlyMethod {
  year: string;
  topup_method: string;
  total_topups: number;
  total_amount: number;
}

export interface TopupMonthAmount {
  month: string;
  total_amount: number;
}

export interface TopupYearlyAmount {
  year: string;
  total_amount: number;
}
