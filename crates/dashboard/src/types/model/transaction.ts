export interface Transaction {
  id: number;
  card_number: string;
  amount: number;
  merchant_id: number;
  payment_method: string;
  transaction_time: string;
  created_at: string;
  updated_at: string;
}

export interface TransactionTrashed extends Transaction {
  deleted_at: string;
}

export interface TransactionMonthStatusSuccess {
  year: string;
  month: string;
  total_amount: number;
  total_success: number;
}

export interface TransactionYearStatusSuccess {
  year: string;
  total_amount: number;
  total_success: number;
}

export interface TransactionMonthStatusFailed {
  year: string;
  month: string;
  total_amount: number;
  total_failed: number;
}

export interface TransactionYearStatusFailed {
  year: string;
  total_amount: number;
  total_failed: number;
}

export interface TransactionMonthMethod {
  month: string;
  payment_method: string;
  total_transactions: number;
  total_amount: number;
}

export interface TransactionYearMethod {
  year: string;
  payment_method: string;
  total_transactions: number;
  total_amount: number;
}

export interface TransactionMonthAmount {
  month: string;
  total_amount: number;
}

export interface TransactionYearlyAmount {
  year: string;
  total_amount: number;
}
