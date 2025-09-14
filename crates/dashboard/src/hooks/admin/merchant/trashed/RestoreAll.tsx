import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalMerchantTrashed from "@/store/merchant/trashed/modal";
import useMerchantTrashedStore from "@/store/merchant/trashed/trashed";

export default function useRestoreAllMerchant() {
  const { isModalVisibleRestoreAll, showModalRestoreAll, hideModalRestoreAll } =
    useModalMerchantTrashed();

  const {
    restoreMerchantAllTrashed,
    setLoadingRestoreAllMerchantTrashed,
    loadingRestoreAllMerchantTrashed,
    setErrorRestoreAllMerchantTrashed,
  } = useMerchantTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreAllMerchantTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await restoreMerchantAllTrashed(toast);

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
          description: "Gagal restore Merchant. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat restore Merchant";
      setErrorRestoreAllMerchantTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreAllMerchantTrashed(false);
      hideModalRestoreAll();
    }
  };

  return {
    handleSubmit,
    loadingRestoreAllMerchantTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  };
}
