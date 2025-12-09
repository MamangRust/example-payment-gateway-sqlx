import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useMerchantTrashedStore from "@/store/merchant/trashed/trashed";
import useModalMerchantTrashed from "@/store/merchant/trashed/modal";
import { RestoreMerchantTrashed } from "@/types/domain/request";

export default function useRestoreMerchant() {
  const {
    restoreMerchantId,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useModalMerchantTrashed();

  const {
    restoreMerchantTrashed,
    setLoadingRestoreMerchantTrashed,
    loadingRestoreMerchantTrashed,
    setErrorRestoreMerchantTrashed,
  } = useMerchantTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreMerchantTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: RestoreMerchantTrashed = {
        id: restoreMerchantId as number,
        toast: toast,
      };

      const result = await restoreMerchantTrashed(req);

      if (result) {
        toast({
          title: "Success",
          description: "Merchant berhasil direstore",
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
      setErrorRestoreMerchantTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreMerchantTrashed(false);
      hideModalRestore();
    }
  };

  return {
    restoreMerchantId,
    handleSubmit,
    loadingRestoreMerchantTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  };
}
