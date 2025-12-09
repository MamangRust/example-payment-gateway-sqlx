import { useEffect, useRef } from "react";
import useTransactionStore from "@/store/transaction/transaction";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import {
  UpdateTransactionFormValues,
  updateTransactionRequestSchema,
} from "@/schemas";
import { z } from "zod";
import useModalTransaction from "@/store/transaction/modal";
import {
  FindyByIdTransaction,
  UpdateTransaction,
} from "@/types/domain/request";
import useMerchantStore from "@/store/merchant/merchant";

export default function useUpdateTransaction() {
  const {
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
    editTransactionId,
  } = useModalTransaction();

  const { merchants } = useMerchantStore();
  const {
    updateTransaction,
    setLoadingUpdateTransaction,
    loadingUpdateTransaction,
    setErrorUpdateTransaction,

    transaction,
    findByIdTransaction,
  } = useTransactionStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const formRef = useRef<HTMLFormElement>(null);

  useEffect(() => {
    if (isModalVisibleEdit && editTransactionId != null) {
      const req: FindyByIdTransaction = {
        toast,
        id: editTransactionId,
      };

      findByIdTransaction(req);
    }
  }, [isModalVisibleEdit, editTransactionId]);

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: UpdateTransactionFormValues) => {
    setLoadingUpdateTransaction(true);

    try {
      const validatedValues = updateTransactionRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const selectedMerchant = merchants?.find(
        (m) => m.id === validatedValues.merchant_id.value,
      );

      if (!selectedMerchant) {
        throw new Error("Merchant tidak ditemukan");
      }

      const req: UpdateTransaction = {
        id: editTransactionId as number,
        card_number: validatedValues.card_number.value,
        amount: validatedValues.amount,
        payment_method: validatedValues.payment_method,
        merchant_id: validatedValues.merchant_id.value,
        transaction_time: validatedValues.transaction_time,
        api_key: selectedMerchant.api_key,
        toast: toast,
      };

      const result = await updateTransaction(req);

      if (result) {
        toast({
          title: "Success",
          description: "User berhasil diupdate",
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
        setErrorUpdateTransaction(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorUpdateTransaction(
          error?.message || "Terjadi kesalahan saat mengedit user",
        );
        toast({
          title: "Error",
          description: error?.message || "Terjadi kesalahan saat mengedit user",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingUpdateTransaction(false);
      hideModalEdit();
    }
  };

  return {
    transaction,
    editTransactionId,
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingUpdateTransaction,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  };
}
