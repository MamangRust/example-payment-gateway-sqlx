import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { DeletePermanentMerchant } from "@/types/domain/request";
import useModalMerchantTrashed from "@/store/merchant/trashed/modal";
import useMerchantTrashedStore from "@/store/merchant/trashed/trashed";

export default function useDeletePermanentMerchant() {
  const {
    deletePermanentMerchantId,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useModalMerchantTrashed();

  const {
    deletePermanentMerchant,
    setLoadingDeletePermanentMerchantTrashed,
    loadingDeletePermanentMerchantTrashed,
    setErrorDeletePermanentMerchantTrashed,
  } = useMerchantTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentMerchantTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: DeletePermanentMerchant = {
        id: deletePermanentMerchantId as number,
        toast: toast,
      };

      const result = await deletePermanentMerchant(req);

      if (result) {
        toast({
          title: "Success",
          description: "Merchant berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Merchant. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Merchant";
      setErrorDeletePermanentMerchantTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentMerchantTrashed(false);
      hideModalDeletePermanent();
    }
  };

  return {
    deletePermanentMerchantId,
    handleSubmit,
    loadingDeletePermanentMerchantTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  };
}
