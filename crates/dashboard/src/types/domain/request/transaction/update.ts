export interface UpdateTransaction {
  id: number;
  card_number: string;
  amount: number;
  payment_method: string;
  merchant_id: number;
  api_key: string;
  transaction_time: Date;
  toast: any;
}
