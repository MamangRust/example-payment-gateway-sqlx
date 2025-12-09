import { UpdateMerchantFormValues } from "@/schemas";

export interface MerchantUpdateFormProps {
  onSubmit: (data: UpdateMerchantFormValues) => void;
  defaultValues?: UpdateMerchantFormValues;
}
