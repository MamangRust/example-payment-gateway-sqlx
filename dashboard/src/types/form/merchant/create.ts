import { CreateMerchantFormValues } from "@/schemas";

export interface MerchantCreateFormProps {
  onSubmit: (data: CreateMerchantFormValues) => void;
}
