export interface Card {
  id: number;
  user_id: number;
  card_number: string;
  card_type: string;
  expire_date: Date;
  cvv: string;
  card_provider: string;
  created_at: string;
  updated_at: string;
}

export interface CardTrashed extends Card {
  deleted_at: string;
}

export interface DashboardCard {
  total_balance?: number | null;
  total_topup?: number | null;
  total_withdraw?: number | null;
  total_transaction?: number | null;
  total_transfer?: number | null;
}

export interface DashboardCardCardNumber {
  total_balance?: number | null;
  total_topup?: number | null;
  total_withdraw?: number | null;
  total_transaction?: number | null;
  total_transfer_send?: number | null;
  total_transfer_receiver?: number | null;
}

export interface CardMonthBalance {
  month: string;
  total_balance: number;
}

export interface CardYearlyBalance {
  year: string;
  total_balance: number;
}

export interface CardMonthTopupAmount {
  month: string;
  total_amount: number;
}

export interface CardYearlyTopupAmount {
  year: string;
  total_amount: number;
}

export interface CardMonthTransactionAmount {
  month: string;
  total_amount: number;
}

export interface CardYearlyTransactionAmount {
  year: string;
  total_amount: number;
}

export interface CardMonthTransferAmount {
  month: string;
  total_amount: number;
}

export interface CardYearlyTransferAmount {
  year: string;
  total_amount: number;
}

export interface CardMonthWithdrawAmount {
  month: string;
  total_amount: number;
}

export interface CardYearlyWithdrawAmount {
  year: string;
  total_amount: number;
}
