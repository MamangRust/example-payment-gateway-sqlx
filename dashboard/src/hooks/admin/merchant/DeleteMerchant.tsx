import useMerchantStore from "@/store/merchant/merchant";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { z } from "zod";
import useModalMerchant from "@/store/merchant/modal";
import { FindTrashedMerchant } from "@/types/domain/request";

export default function useDeleteMerchant() {
  const {
    deleteMerchantId,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useModalMerchant();

  const {
    trashedMerchant,
    setLoadingTrashedMerchant,
    loadingTrashedMerchant,
    setErrorTrashedMerchant,
  } = useMerchantStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingTrashedMerchant(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: FindTrashedMerchant = {
        id: deleteMerchantId as number,
        toast: toast,
      };

      const result = await trashedMerchant(req);

      if (result) {
        toast({
          title: "Success",
          description: "Merchant berhasil diupdate",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal membuat merchant. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorTrashedMerchant(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorTrashedMerchant(
          error?.message || "Terjadi kesalahan saat menghapus merchant",
        );
        toast({
          title: "Error",
          description:
            error?.message || "Terjadi kesalahan saat menghapus merchant",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingTrashedMerchant(false);
      hideModalDelete();
    }
  };

  return {
    deleteMerchantId,
    handleSubmit,
    loadingTrashedMerchant,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  };
}
