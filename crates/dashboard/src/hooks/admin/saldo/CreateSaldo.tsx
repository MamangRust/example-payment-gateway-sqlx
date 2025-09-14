import { useRef } from "react";
import useSaldoStore from "@/store/saldo/saldo";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { CreateSaldoFormValues, createSaldoRequestSchema } from "@/schemas";
import { z } from "zod";
import useModalSaldo from "@/store/saldo/modal";
import { CreateSaldo } from "@/types/domain/request";

export default function useCreateSaldo() {
  const { isModalVisible, showModal, hideModal } = useModalSaldo();
  const formRef = useRef<HTMLFormElement>(null);

  const {
    createSaldo,
    setLoadingCreateSaldo,
    loadingCreateSaldo,
    setErrorCreateSaldo,
  } = useSaldoStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: CreateSaldoFormValues) => {
    setLoadingCreateSaldo(true);

    try {
      const validatedValues = createSaldoRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: CreateSaldo = {
        card_number: validatedValues.card_number,
        total_balance: validatedValues.total_balance,
        toast: toast,
      };

      const result = await createSaldo(req);

      if (result) {
        toast({
          title: "Success",
          description: "Saldo berhasil dibuat",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal membuat Saldo. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorCreateSaldo(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorCreateSaldo(
          error?.message || "Terjadi kesalahan saat membuat saldo",
        );
        toast({
          title: "Error",
          description: error?.message || "Terjadi kesalahan saat membuat saldo",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingCreateSaldo(false);
      hideModal();
    }
  };

  return {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateSaldo,
    isModalVisible,
    showModal,
    hideModal,
  };
}
