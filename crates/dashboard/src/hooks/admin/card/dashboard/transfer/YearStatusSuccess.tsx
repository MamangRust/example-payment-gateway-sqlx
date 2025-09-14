import { calculatePercentageChange } from "@/helpers/calculate";
import { debounce } from "@/helpers/debounce";
import useTransferStore from "@/store/transfer/transfer";
import { useCallback } from "react";

export default function useYearStatusSuccessTransfer(currentYear: number, toast: any) {
    const {
        findYearStatusSuccess,
        yearStatusSuccess,
        loadingYearStatusSuccess,
        setLoadingYearStatusSuccess,
        setErrorYearStatusSuccess,
    } = useTransferStore();

    const currentYearSuccessData = yearStatusSuccess?.find(
        (balance) => balance.year === currentYear.toString(),
    );

    const previousYearSuccessData = yearStatusSuccess?.find(
        (balance) => balance.year === (currentYear - 1).toString(),
    );

    const currentYearSuccess = currentYearSuccessData?.total_amount || 0;
    const previousYearSucces = previousYearSuccessData?.total_amount || 0;

    const yearPercentageSuccessChange = calculatePercentageChange(
        currentYearSuccess,
        previousYearSucces,
    );

    const fetchYearlySuccess = useCallback(
        async (year: number) => {
            try {
                setLoadingYearStatusSuccess(true);
                setErrorYearStatusSuccess(null);

                await findYearStatusSuccess(toast, year);
            } catch (error) {
                setErrorYearStatusSuccess("Failed to fetch yearly balance");
            } finally {
                setLoadingYearStatusSuccess(false);
            }
        },
        [
            findYearStatusSuccess,
            setLoadingYearStatusSuccess,
            setLoadingYearStatusSuccess,
        ],
    );

    const debouncedFetchYearlySuccess = debounce(fetchYearlySuccess, 300);

    return {
        debouncedFetchYearlySuccess,
        loadingYearStatusSuccess,
        currentYearSuccessData,
        previousYearSuccessData,
        currentYearSuccess,
        previousYearSucces,
        yearPercentageSuccessChange,
        findYearStatusSuccess
    }



}