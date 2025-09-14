export interface Merchant {
  id: number;
  name: string;
  api_key: string;
  user_id: number;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface MerchantTrashed extends Merchant {
  deleted_at: string;
}

export interface MerchantTransaction {
  id: number;
  card_number: string;
  amount: number;
  payment_method: string;
  merchant_id: number;
  merchant_name: string;
  transaction_time: string;
  created_at: string;
  updated_at: string;
  deleted_at: string;
}

export interface MerchantMonthlyPaymentMethod {
  month: string;
  payment_method: string;
  total_amount: number;
}

export interface MerchantYearlyPaymentMethod {
  year: string;
  payment_method: string;
  total_amount: number;
}

export interface MerchantMonthlyAmount {
  month: string;
  total_amount: number;
}

export interface MerchantYearlyAmount {
  year: string;
  total_amount: number;
}

export interface MerchantMonthlyTotalAmount {
  month: string;
  year: string;
  total_amount: number;
}

export interface MerchantYearlyTotalAmount {
  year: string;
  total_amount: number;
}
