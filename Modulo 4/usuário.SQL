-- criar usuário
INSERT INTO dbo.[User](UserName, Email) VALUES ('joao.silva','joao@ex.com');

-- criar processo
DECLARE @pid INT;
EXEC dbo.sp_CreateProcess @ProcessName = 'Onboarding - Cliente', @InitiatorUserId = 1, @OutProcessId = @pid OUTPUT;

-- adicionar tarefa
DECLARE @tid INT;
EXEC dbo.sp_AddTask @ProcessId = @pid, @TaskName = 'Revisar documentação', @TaskType = 'HumanTask', @AssignedToUserId = 1, @Payload = '{"documentId":42}', @OutTaskId = @tid OUTPUT;

-- iniciar e completar
EXEC dbo.sp_StartTask @TaskId = @tid, @UserId = 1;
EXEC dbo.sp_CompleteTask @TaskId = @tid, @UserId = 1, @ResultPayload = '{"approved":true,"note":"OK"}';

-- finalizar processo (após lógica que verifique todas as tasks)
EXEC dbo.sp_CompleteProcess @ProcessId = @pid;
