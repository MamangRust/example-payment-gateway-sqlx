export interface Saldo {
  id: number;
  card_number: string;
  total_balance: number;
  withdraw_time: string;
  withdraw_amount: number;
  created_at: string;
  updated_at: string;
}

export interface SaldoTrashed {
  id: number;
  card_number: string;
  total_balance: number;
  withdraw_time: string;
  withdraw_amount: number;
  created_at: string;
  updated_at: string;
  deleted_at: string;
}

export interface SaldoMonthTotalBalance {
  month: string;
  year: string;
  total_balance: number;
}

export interface SaldoYearTotalBalance {
  year: string;
  total_balance: number;
}

export interface SaldoMonthBalance {
  month: string;
  total_balance: number;
}

export interface SaldoYearBalance {
  year: string;
  total_balance: number;
}
