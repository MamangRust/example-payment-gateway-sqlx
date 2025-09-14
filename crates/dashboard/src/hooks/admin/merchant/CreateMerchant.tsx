import useMerchantStore from "@/store/merchant/merchant";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import {
  CreateMerchantFormValues,
  createMerchantRequestSchema,
} from "@/schemas";
import { z } from "zod";
import useModalMerchant from "@/store/merchant/modal";
import { CreateMerchant } from "@/types/domain/request";
import { useRef } from "react";

export default function useCreateMerchant() {
  const formRef = useRef<HTMLFormElement>(null);
  const { isModalVisible, showModal, hideModal } = useModalMerchant();

  const {
    createMerchant,
    setLoadingCreateMerchant,
    loadingCreateMerchant,
    setErrorCreateMerchant,
  } = useMerchantStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: CreateMerchantFormValues) => {
    setLoadingCreateMerchant(true);

    try {
      const validatedValues = createMerchantRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: CreateMerchant = {
        user_id: validatedValues.user_id,
        name: validatedValues.name,
        toast: toast,
      };

      const result = await createMerchant(req);

      if (result) {
        toast({
          title: "Success",
          description: "Merchant berhasil dibuat",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal membuat Merchant. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorCreateMerchant(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorCreateMerchant(
          error?.message || "Terjadi kesalahan saat membuat Merchant",
        );
        toast({
          title: "Error",
          description:
            error?.message || "Terjadi kesalahan saat membuat Merchant",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingCreateMerchant(false);
      hideModal();
    }
  };

  return {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateMerchant,
    isModalVisible,
    showModal,
    hideModal,
  };
}
