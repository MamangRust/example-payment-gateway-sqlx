import useSaldoStore from "@/store/saldo/saldo";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalSaldo from "@/store/saldo/modal";
import { TrashedSaldo } from "@/types/domain/request";

export default function useDeleteSaldo() {
  const {
    deleteSaldoId,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useModalSaldo();

  const {
    trashedSaldo,
    setLoadingTrashedSaldo,
    loadingTrashedSaldo,
    setErrorTrashedSaldo,
  } = useSaldoStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingTrashedSaldo(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: TrashedSaldo = {
        id: deleteSaldoId as number,
        toast: toast,
      };

      const result = await trashedSaldo(req);

      if (result) {
        toast({
          title: "Success",
          description: "Saldo berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus saldo. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus saldo";
      setErrorTrashedSaldo(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingTrashedSaldo(false);
      hideModalDelete();
    }
  };

  return {
    deleteSaldoId,
    handleSubmit,
    loadingTrashedSaldo,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  };
}
