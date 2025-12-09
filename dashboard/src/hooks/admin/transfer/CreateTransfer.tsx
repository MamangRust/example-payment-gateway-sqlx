import { useRef } from "react";
import useTransferStore from "@/store/transfer/transfer";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import {
  CreateTransferFormValues,
  createTransferRequestSchema,
} from "@/schemas";
import { z } from "zod";
import useModalTransfer from "@/store/transfer/modal";
import { CreateTransfer } from "@/types/domain/request";

export default function useCreateTransfer() {
  const { isModalVisible, showModal, hideModal } = useModalTransfer();

  const {
    createTransfer,
    setLoadingCreateTransfer,
    loadingCreateTransfer,
    setErrorCreateTransfer,
  } = useTransferStore();
  const formRef = useRef<HTMLFormElement>(null);
  const navigate = useNavigate();
  const { toast } = useToast();

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: CreateTransferFormValues) => {
    setLoadingCreateTransfer(true);

    try {
      const validatedValues = createTransferRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: CreateTransfer = {
        transfer_from: validatedValues.transfer_from,
        transfer_to: validatedValues.transfer_to,
        transfer_amount: validatedValues.transfer_amount,
        toast: toast,
      };

      const result = await createTransfer(req);

      if (result) {
        toast({
          title: "Success",
          description: "Transfer berhasil dibuat",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal membuat Transfer. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorCreateTransfer(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorCreateTransfer(
          error?.message || "Terjadi kesalahan saat membuat Transfer",
        );
        toast({
          title: "Error",
          description:
            error?.message || "Terjadi kesalahan saat membuat Transfer",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingCreateTransfer(false);
      hideModal();
    }
  };

  return {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateTransfer,
    isModalVisible,
    showModal,
    hideModal,
  };
}
