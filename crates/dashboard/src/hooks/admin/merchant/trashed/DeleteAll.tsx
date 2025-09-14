import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalMerchantTrashed from "@/store/merchant/trashed/modal";
import useMerchantTrashedStore from "@/store/merchant/trashed/trashed";

export default function useDeletePermanentAllMerchant() {
  const {
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useModalMerchantTrashed();

  const {
    deletePermanentAllMerchant,
    setLoadingDeletePermanentAllMerchantTrashed,
    loadingDeletePermanentAllMerchantTrashed,
    setErrorDeletePermanentAllMerchantTrashed,
  } = useMerchantTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentAllMerchantTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await deletePermanentAllMerchant(toast);

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
      setErrorDeletePermanentAllMerchantTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentAllMerchantTrashed(false);
      hideModalDeletePermanentAll();
    }
  };

  return {
    handleSubmit,
    loadingDeletePermanentAllMerchantTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  };
}
