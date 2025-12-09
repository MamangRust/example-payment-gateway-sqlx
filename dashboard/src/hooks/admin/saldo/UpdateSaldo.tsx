import { useEffect, useRef } from "react";
import useSaldoStore from "@/store/saldo/saldo";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { UpdateSaldoFormValues, updateSaldoRequestSchema } from "@/schemas";
import { z } from "zod";
import useModalSaldo from "@/store/saldo/modal";
import { FindByIdSaldo, UpdateSaldo } from "@/types/domain/request";

export default function useUpdateSaldo() {
  const { isModalVisibleEdit, showModalEdit, hideModalEdit, editSaldoId } =
    useModalSaldo();
  const formRef = useRef<HTMLFormElement>(null);

  const {
    updateSaldo,
    setLoadingUpdateSaldo,
    loadingUpdateSaldo,
    setErrorUpdateSaldo,

    findByIdSaldo,
    saldo,
  } = useSaldoStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  useEffect(() => {
    if (isModalVisibleEdit && editSaldoId !== null) {
      const req: FindByIdSaldo = {
        toast,
        id: editSaldoId,
      };

      findByIdSaldo(req);
    }
  }, [isModalVisibleEdit, editSaldoId]);

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: UpdateSaldoFormValues) => {
    setLoadingUpdateSaldo(true);

    try {
      const validatedValues = updateSaldoRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: UpdateSaldo = {
        id: editSaldoId as number,
        card_number: validatedValues.card_number.value,
        total_balance: validatedValues.total_balance,
        toast: toast,
      };

      console.log("req", req);

      const result = await updateSaldo(req);

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
          description: "Gagal membuat saldo. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorUpdateSaldo(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorUpdateSaldo(
          error?.message || "Terjadi kesalahan saat mengedit saldo",
        );
        toast({
          title: "Error",
          description:
            error?.message || "Terjadi kesalahan saat mengedit saldo",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingUpdateSaldo(false);
      hideModalEdit();
    }
  };

  return {
    saldo,
    editSaldoId,
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingUpdateSaldo,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  };
}
