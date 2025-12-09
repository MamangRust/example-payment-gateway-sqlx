import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  CreateWithdrawFormValues,
  createWithdrawRequestSchema,
} from "@/schemas";
import { zodResolver } from "@hookform/resolvers/zod";
import { useToast } from "@/hooks/use-toast";
import useCardStore from "@/store/card/card";
import { debounce } from "@/helpers/debounce";
import { forwardRef, useCallback, useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import AsyncSelect from "react-select/async";
import { WithdrawCreateFormProps } from "@/types/form";
import { FindAllCard } from "@/types/domain/request";

interface CardOption {
  value: string;
  label: string;
}

const CreateWithdrawForm = forwardRef<HTMLFormElement, WithdrawCreateFormProps>(
  ({ onSubmit }, ref) => {
    const {
      register,
      handleSubmit,
      control,
      formState: { errors },
    } = useForm<CreateWithdrawFormValues>({
      resolver: zodResolver(createWithdrawRequestSchema),
      defaultValues: {
        card_number: "",
        withdraw_amount: 0,
        withdraw_time: new Date(),
      },
    });

    const { toast } = useToast();

    const { cards, loadingGetCards, setLoadingGetCards, findAllCards } =
      useCardStore();

    const [cardOptions, setCardOptions] = useState<CardOption[]>([]);

    useEffect(() => {
      const fetchInitialCards = async () => {
        setLoadingGetCards(true);
        try {
          const searchReq: FindAllCard = {
            search: "",
            page: 1,
            page_size: 10,
            toast: toast,
          };
          await findAllCards(searchReq);

          if (cards) {
            const options = cards.map((card) => ({
              value: card.card_number,
              label: card.card_number,
            }));

            console.log("card", cards);
            setCardOptions(options);
          }
        } catch (error) {
          console.error("Failed to fetch initial cards:", error);
        } finally {
          setLoadingGetCards(false);
        }
      };

      fetchInitialCards();
    }, []);

    const loadOptionCards = useCallback(
      debounce(
        async (
          inputValue: string,
          callback: (options: CardOption[]) => void,
        ) => {
          setLoadingGetCards(true);
          try {
            const searchReq: FindAllCard = {
              search: inputValue,
              page: 1,
              page_size: 10,
              toast: toast,
            };
            await findAllCards(searchReq);

            if (cards) {
              const filteredOptions = cards
                .filter((card) =>
                  card.card_number
                    .toLowerCase()
                    .includes(inputValue.toLowerCase()),
                )
                .map((card) => ({
                  value: card.card_number,
                  label: card.card_number,
                }));

              callback(filteredOptions);
            }
          } catch (error) {
            console.error("Failed to fetch cards:", error);
          } finally {
            setLoadingGetCards(false);
          }
        },
        500,
      ),
      [cards, findAllCards, toast],
    );

    return (
      <form ref={ref} onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <div>
          <Label htmlFor="cardNumber" className="text-sm font-medium">
            Card Number
          </Label>
          <Controller
            name="card_number"
            control={control}
            render={({ field }) => (
              <AsyncSelect
                {...field}
                loadOptions={loadOptionCards}
                defaultOptions={cardOptions}
                isLoading={loadingGetCards}
                placeholder="Select a card"
                className="mt-1"
                styles={{
                  control: (base) => ({
                    ...base,
                    borderColor: errors.card_number
                      ? "#ef4444"
                      : base.borderColor,
                    borderRadius: "0.375rem",
                    padding: "0.25rem",
                    boxShadow: "none",
                    "&:hover": {
                      borderColor: "#3b82f6",
                    },
                  }),
                  option: (base, { isFocused, isSelected }) => ({
                    ...base,
                    backgroundColor: isSelected
                      ? "#3b82f6"
                      : isFocused
                        ? "#bfdbfe"
                        : "white",
                    color: isSelected ? "white" : "black",
                    "&:hover": {
                      backgroundColor: "#bfdbfe",
                    },
                  }),
                  menu: (base) => ({
                    ...base,
                    borderRadius: "0.375rem",
                    boxShadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)",
                  }),
                }}
                value={cardOptions.find(
                  (option) => option.value === field.value,
                )}
                onChange={(selectedOption) => {
                  field.onChange(selectedOption ? selectedOption.value : null);
                }}
              />
            )}
          />
          {errors.card_number && (
            <p className="text-red-500 text-sm mt-1">
              {errors.card_number.message}
            </p>
          )}
        </div>
        <div>
          <Label htmlFor="withdraw_amount" className="text-sm font-medium">
            Withdraw Amount
          </Label>
          <Input
            id="withdraw_amount"
            type="number"
            placeholder="Enter withdraw amount"
            className="mt-1"
            {...register("withdraw_amount", { valueAsNumber: true })}
          />
          {errors.withdraw_amount && (
            <p className="text-red-500 text-sm mt-1">
              {errors.withdraw_amount.message}
            </p>
          )}
        </div>
        <div>
          <Label htmlFor="withdraw_time" className="text-sm font-medium">
            Withdraw Time
          </Label>
          <Controller
            name="withdraw_time"
            control={control}
            render={({ field }) => {
              const value = new Date(field.value).toISOString().slice(0, 16);

              return (
                <Input
                  id="withdraw_time"
                  type="datetime-local"
                  className="mt-1"
                  value={value}
                  onChange={(e) => {
                    field.onChange(new Date(e.target.value));
                  }}
                />
              );
            }}
          />
          {errors.withdraw_time && (
            <p className="text-red-500 text-sm mt-1">
              {errors.withdraw_time.message}
            </p>
          )}
        </div>
      </form>
    );
  },
);

CreateWithdrawForm.displayName = "CreateWithdrawForm";

export default CreateWithdrawForm;
