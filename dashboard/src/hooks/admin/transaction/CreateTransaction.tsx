import { useRef } from "react";
import useTransactionStore from "@/store/transaction/transaction";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import {
  CreateTransactionFormValues,
  createTransactionRequestSchema,
} from "@/schemas";
import { z } from "zod";
import useModalTransaction from "@/store/transaction/modal";
import { CreateTransaction } from "@/types/domain/request";
import useMerchantStore from "@/store/merchant/merchant";

export default function useCreateTransaction() {
  const { isModalVisible, showModal, hideModal } = useModalTransaction();
  const formRef = useRef<HTMLFormElement>(null);
  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const { merchants } = useMerchantStore();
  const {
    createTransaction,
    setLoadingCreateTransaction,
    loadingCreateTransaction,
    setErrorCreateTransaction,
  } = useTransactionStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async (data: CreateTransactionFormValues) => {
    setLoadingCreateTransaction(true);

    try {
      const validatedValues = createTransactionRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const selectedMerchant = merchants?.find(
        (m) => m.id === validatedValues.merchant_id,
      );

      if (!selectedMerchant) {
        throw new Error("Merchant tidak ditemukan");
      }

      const req: CreateTransaction = {
        card_number: validatedValues.card_number,
        amount: validatedValues.amount,
        payment_method: validatedValues.payment_method,
        merchant_id: validatedValues.merchant_id,
        transaction_time: validatedValues.transaction_time,
        api_key: selectedMerchant.api_key,
        toast: toast,
      };

      console.log("req", req);

      const result = await createTransaction(req);

      if (result) {
        toast({
          title: "Success",
          description: "Transaction berhasil dibuat",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal membuat user. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorCreateTransaction(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorCreateTransaction(
          error?.message || "Terjadi kesalahan saat membuat transaction",
        );
        toast({
          title: "Error",
          description:
            error?.message || "Terjadi kesalahan saat membuat transaction",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingCreateTransaction(false);
      hideModal();
    }
  };

  return {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateTransaction,
    isModalVisible,
    showModal,
    hideModal,
  };
}
