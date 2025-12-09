import { useEffect, useRef } from "react";
import useTopupStore from "@/store/topup/topup";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { UpdateTopupFormValues, updateTopupRequestSchema } from "@/schemas";
import { z } from "zod";
import useModalTopup from "@/store/topup/modal";
import { FindByIdTopup, UpdateTopup } from "@/types/domain/request/topup";

export default function useUpdateTopup() {
  const { isModalVisibleEdit, showModalEdit, hideModalEdit, editTopupId } =
    useModalTopup();

  const {
    updateTopup,
    setLoadingUpdateTopup,
    loadingUpdateTopup,
    setErrorUpdateTopup,

    topup,
    findByIdTopup,
  } = useTopupStore();

  const navigate = useNavigate();
  const { toast } = useToast();
  const formRef = useRef<HTMLFormElement>(null);

  useEffect(() => {
    if (isModalVisibleEdit && editTopupId != null) {
      const req: FindByIdTopup = {
        toast,
        id: editTopupId,
      };

      findByIdTopup(req);
    }
  }, [isModalVisibleEdit, editTopupId]);

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: UpdateTopupFormValues) => {
    setLoadingUpdateTopup(true);

    try {
      const validatedValues = updateTopupRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: UpdateTopup = {
        id: editTopupId as number,
        card_number: validatedValues.card_number.value,
        topup_amount: validatedValues.topup_amount,
        topup_method: validatedValues.topup_method,
        toast: toast,
      };

      const result = await updateTopup(req);

      if (result) {
        toast({
          title: "Success",
          description: "topup berhasil diupdate",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal membuat topup. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorUpdateTopup(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorUpdateTopup(
          error?.message || "Terjadi kesalahan saat mengedit topup",
        );
        toast({
          title: "Error",
          description:
            error?.message || "Terjadi kesalahan saat mengedit topup",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingUpdateTopup(false);
      hideModalEdit();
    }
  };

  return {
    topup,
    editTopupId,
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingUpdateTopup,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  };
}
