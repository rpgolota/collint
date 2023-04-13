clear all
format long
set(0, 'DefaultLineLineWidth', 3)

global M p tmax

% Correct answer for ROBETDNALG
%                    7239056481

%% Initialization
M = 100; % Group size
p = 0.7;
t = 0;
tmax = 10000;

for i = 1:M
    
    y(i,:) = agent(i);
    cost(i,:) = abs(y(i).guess - 7239056481);
    
end

m = find(cost == min(cost)); % agent with initial lowest cost score --> returns index so need leader board
    
while any(cost==0, 'all') == 0 && t < tmax
    
    % Calculate imitation probability of all agents
    V = rand(M,1);
    P = (V > p); % 1 if V > p and 0 if V <= p
    
    for i = 1:100
        if i == m
            P(m) = 1;
        end 
        %% Elementary move 1-p 
        if P(i) == 1
            k = randperm(10, 2); % swap index
            y(i,:).swap(k(1),k(2)) % Can cause agent with min cost to increase cost 
            if y(i).cost < y(m).cost % set new best guess
                m = i;
            end
            
            %% Imitation p
        else
            if y(m).cost < y(i).cost % if i = m then nothing happens
                k = randi([1 10],1); % imitation index
                y(i,:).imitate(k, y(m).guess)
                if y(i).cost < y(m).cost % set new best guess
                    m = i;
                end
            end
        end
        cost(i,:) = abs(y(i).guess - 7239056481);
    end
    
    %fprintf('%.0f\n',y(m).guess);
    minCost(t+1,:) = y(m).cost;
    t = t + 1;
end

t_star = t;
C = M*t_star / factorial(10);
fprintf('total time: %f\n',t_star)
fprintf('Computational cost: %f\n',C)
plot([1:t],minCost)
ylabel("Minimum cost")
xlabel("time")

if t == tmax
    warning('Max Iterations reached')  
else 
    fprintf('Success.')
end